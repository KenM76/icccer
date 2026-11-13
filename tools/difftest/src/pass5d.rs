//! # Pass 5d — the SOURCE-side black point (ISO/CD 18619 4.2.3), cross-checked
//!
//! `Chain::with_bpc()` estimates a LUT source's black by ISO/CD 18619 4.2.3.
//! This module compares that estimate with the pinned lcms2's
//! `cmsDetectBlackPoint` — **called directly** through
//! `probes/lcms2_black_point.c`, so no round trip or recovery stands between
//! the oracle and the number — and then prices the difference end to end
//! through a CMYK → sRGB soft-proof chain. Record: README §29; tolerances:
//! `docs/TOLERANCES.md` §3.5.10.
//!
//! Evidence class of every row: **cross-check** (icccer vs lcms2) or
//! **self-consistency** (icccer BPC on vs off). There is no published value for
//! any black point here (A27/A42), so no row is ground truth.

use std::path::{Path, PathBuf};
use std::process::Command;

use icccer_cmm::bpc::PERCEPTUAL_BLACK;
use icccer_cmm::matrix_trc::Intent as CmmIntent;
use icccer_cmm::transform::{Chain, Destination};
use icccer_color::{D50, Lab, Xyz};
use icccer_profile::Profile;

use crate::{Bpc, Intent, Kind, Metric, Oracle, Precalc, Record, Request, Space, Tolerance};

const SWOP: &str = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";
const SRGB_SYS: &str = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";
const SRC: &str = "README §29 / TOLERANCES.md §3.5.10, both sides computed in this run";

fn mab() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/synthetic/v4-cmyk-mab-lab.icc")
}

fn probe_exe() -> PathBuf {
    std::env::var_os("ICCCER_LCMS2_BP").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor/build-msvc/lcms2_black_point.exe"),
        PathBuf::from,
    )
}

/// One `cmsDetectBlackPoint` result, as the probe prints it.
#[derive(Debug, Clone)]
pub struct Lcms2Black {
    pub lab: Lab,
    pub branch: String,
}

fn run_probe(args: &[String]) -> Result<String, String> {
    let out = Command::new(probe_exe())
        .args(args)
        .output()
        .map_err(|e| format!("lcms2_black_point: {e} (build: probes/build-probe.ps1)"))?;
    if !out.status.success() {
        return Err(format!("lcms2_black_point exit {:?}", out.status.code()));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn lcms2_black(profile: &Path, intent: Intent) -> Result<Lcms2Black, String> {
    let line = run_probe(&[profile.display().to_string(), intent.as_num().to_string()])?;
    let field = |k: &str| -> Option<&str> {
        line.split('\t').find_map(|kv| kv.strip_prefix(k).and_then(|r| r.strip_prefix('=')))
    };
    let num = |k: &str| field(k).and_then(|v| v.parse::<f64>().ok()).ok_or(format!("no {k}"));
    Ok(Lcms2Black {
        lab: Lab { l: num("L")?, a: num("a")?, b: num("b")? },
        branch: field("branch").unwrap_or("?").to_string(),
    })
}

/// lcms2's device→Lab leg (`--lab`) or Lab→device leg (`--b2a`).
fn lcms2_leg(mode: &str, profile: &Path, intent: Intent, v: &[f64]) -> Result<Vec<f64>, String> {
    let mut args = vec![format!("--{mode}"), profile.display().to_string()];
    args.push(intent.as_num().to_string());
    args.extend(v.iter().map(|x| format!("{x:.12}")));
    let line = run_probe(&args)?;
    line.split('\t').map(|s| s.parse::<f64>().map_err(|e| e.to_string())).collect()
}

fn cmm_intent(i: Intent) -> CmmIntent {
    match i {
        Intent::Perceptual => CmmIntent::Perceptual,
        Intent::RelativeColorimetric => CmmIntent::MediaRelative,
        Intent::Saturation => CmmIntent::Saturation,
        Intent::AbsoluteColorimetric => CmmIntent::Absolute,
    }
}

fn load(p: &Path) -> Result<Profile, String> {
    let bytes = std::fs::read(p).map_err(|e| format!("{}: {e}", p.display()))?;
    Profile::parse(&bytes).map_err(|e| e.to_string())
}

/// icccer's source black, by the consumer's own call shape.
fn icccer_black(src: &Profile, intent: Intent) -> Result<Lab, String> {
    let chain = Chain::with_destination(src, Destination::None, cmm_intent(intent))
        .and_then(Chain::with_bpc)
        .map_err(|e| format!("{e:?}"))?;
    let scale = chain.bpc_disclosure().ok_or("no disclosure")?;
    Ok(Lab::from_xyz(scale.source_black(), D50))
}

/// `T(pcs, Lab, Source, Perceptual)` — icccer's B2A0 of a PCS value.
fn icccer_b2a0(src: &Profile, pcs: Xyz) -> Result<Vec<f64>, String> {
    Chain::new(src, src, CmmIntent::Perceptual)
        .and_then(|c| c.pcs_to_destination(pcs))
        .map_err(|e| format!("{e:?}"))
}

pub fn de76(a: Lab, b: Lab) -> f64 {
    ((a.l - b.l).powi(2) + (a.a - b.a).powi(2) + (a.b - b.b).powi(2)).sqrt()
}

pub fn lab_abs_max(a: Lab, b: Lab) -> f64 {
    (a.l - b.l).abs().max((a.a - b.a).abs()).max((a.b - b.b).abs())
}

/// One §A arm: a profile at an intent.
#[derive(Debug, Clone)]
pub struct BlackArm {
    pub name: &'static str,
    pub intent: Intent,
    pub icccer: Lab,
    pub lcms2: Lcms2Black,
}

const ARMS: [(&str, Intent); 6] = [
    ("swop", Intent::RelativeColorimetric),
    ("swop", Intent::Perceptual),
    ("swop", Intent::Saturation),
    ("mab", Intent::RelativeColorimetric),
    ("mab", Intent::Saturation),
    ("mab", Intent::Perceptual),
];

fn arm_path(name: &str) -> PathBuf {
    if name == "swop" { PathBuf::from(SWOP) } else { mab() }
}

fn measure_arm(name: &'static str, intent: Intent) -> Result<BlackArm, String> {
    let p = arm_path(name);
    let src = load(&p)?;
    Ok(BlackArm { name, intent, icccer: icccer_black(&src, intent)?, lcms2: lcms2_black(&p, intent)? })
}

/// The decompositions that say WHY an arm differs (README §29.3).
#[derive(Debug, Clone, Default)]
pub struct Diagnostics {
    /// SWOP: icccer's and lcms2's LocalBlack = B2A0(Lab 0).
    pub swop_lb_icccer: Vec<f64>,
    pub swop_lb_lcms2: Vec<f64>,
    /// lcms2's A2B1 of icccer's LocalBlack: isolates the A2B leg.
    pub swop_lcms2_a2b1_of_icccer_lb: Vec<f64>,
    /// SWOP: lcms2's A2B0 / A2B2 of 400 % ink (its DarkerColorant input).
    pub swop_a2b0_400: Vec<f64>,
    pub swop_a2b2_400: Vec<f64>,
    /// mab: icccer B2A0(Lab 0), icccer B2A0(PERCEPTUAL_BLACK), lcms2's leg.
    pub mab_lb_icccer: Vec<f64>,
    pub mab_lb_icccer_at_prm_black: Vec<f64>,
    pub mab_lb_lcms2: Vec<f64>,
    /// lcms2's A2B1 of icccer's B2A0(PERCEPTUAL_BLACK): reproduces lcms2's
    /// relative-intent black iff its forced BPC is the whole mechanism.
    pub mab_lcms2_a2b1_of_prm_lb: Vec<f64>,
}

fn diagnostics() -> Result<Diagnostics, String> {
    let (swop, m) = (PathBuf::from(SWOP), mab());
    let (s, f) = (load(&swop)?, load(&m)?);
    let zero = Xyz { x: 0.0, y: 0.0, z: 0.0 };
    let mut d = Diagnostics {
        swop_lb_icccer: icccer_b2a0(&s, zero)?,
        swop_lb_lcms2: lcms2_leg("b2a", &swop, Intent::Perceptual, &[0.0, 0.0, 0.0])?,
        swop_a2b0_400: lcms2_leg("lab", &swop, Intent::Perceptual, &[1.0; 4])?,
        swop_a2b2_400: lcms2_leg("lab", &swop, Intent::Saturation, &[1.0; 4])?,
        mab_lb_icccer: icccer_b2a0(&f, zero)?,
        mab_lb_icccer_at_prm_black: icccer_b2a0(&f, PERCEPTUAL_BLACK)?,
        mab_lb_lcms2: lcms2_leg("b2a", &m, Intent::Perceptual, &[0.0, 0.0, 0.0])?,
        ..Diagnostics::default()
    };
    d.swop_lcms2_a2b1_of_icccer_lb =
        lcms2_leg("lab", &swop, Intent::RelativeColorimetric, &d.swop_lb_icccer)?;
    d.mab_lcms2_a2b1_of_prm_lb =
        lcms2_leg("lab", &m, Intent::RelativeColorimetric, &d.mab_lb_icccer_at_prm_black)?;
    Ok(d)
}

/// One §B destination: how icccer and lcms2 each name sRGB.
#[derive(Debug, Clone, Copy)]
pub enum SrgbSide {
    /// icccer `Destination::None` against lcms2 `*sRGB` (the consumer's shape).
    Builtin,
    /// The system sRGB profile file on both sides.
    System,
}

impl SrgbSide {
    pub fn tag(self) -> &'static str {
        match self {
            SrgbSide::Builtin => "builtin-srgb",
            SrgbSide::System => "system-srgb",
        }
    }
}

/// §B's end-to-end numbers for one destination.
#[derive(Debug, Clone)]
pub struct EndToEnd {
    pub side: SrgbSide,
    pub points: usize,
    /// icccer vs lcms2, BPC on: max and p95 of the per-point device max.
    pub on_max: f64,
    pub on_p95: f64,
    /// The same, BPC off (Pass 4's media-relative disagreement, for scale).
    pub off_max: f64,
    pub off_p95: f64,
    /// icccer BPC-on vs BPC-off: how much BPC moves the output.
    pub icccer_bpc_effect_max: f64,
    pub lcms2_bpc_effect_max: f64,
    /// |(icccer on − off) − (lcms2 on − off)|: the BPC increment's disagreement.
    pub increment_max: f64,
    pub increment_p95: f64,
    /// The same increment over the UNCLAMPED population only: points where no
    /// component of any of the four outputs sits at 0 or 1. A clamp erases a
    /// difference on one arm and not the other, so the full-grid increment is
    /// bounded below by the clamp, not by the black point (README §29.4).
    pub increment_unclamped_max: f64,
    pub increment_unclamped_p95: f64,
    pub unclamped_points: usize,
    /// First-order model of the unclamped increment: the BPC-off disagreement
    /// re-seen through the sRGB encoding slope at the BPC-on operating point
    /// (README §29.4). `model_residual_max` is what the model leaves over.
    pub model_increment_max: f64,
    pub model_residual_max: f64,
    /// The rival "icccer silently drops BPC": icccer BPC-off against lcms2 BPC-on.
    pub no_bpc_alt_max: f64,
    /// lcms2 components outside [0,1] before clamping.
    pub lcms2_excursions: usize,
    /// The CMYK operand at which `on_max`, `off_max`, `increment_max` occur.
    pub at_on_max: Vec<f64>,
    pub at_off_max: Vec<f64>,
    pub at_increment_max: Vec<f64>,
}

/// 6⁴ = 1296 CMYK operands, every ink on {0, .2, .4, .6, .8, 1}.
pub fn grid() -> Vec<Vec<f64>> {
    let s = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
    let mut g = Vec::with_capacity(1296);
    for c in s {
        for m in s {
            for y in s {
                for k in s {
                    g.push(vec![c, m, y, k]);
                }
            }
        }
    }
    g
}

fn icccer_rows(src: &Profile, dst: Option<&Profile>, bpc: bool, g: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    let d = dst.map_or(Destination::None, Destination::Profile);
    let mut chain = Chain::with_destination(src, d, CmmIntent::MediaRelative).map_err(|e| format!("{e:?}"))?;
    if bpc {
        chain = chain.with_bpc().map_err(|e| format!("{e:?}"))?;
    }
    g.iter().map(|v| chain.convert(v).map_err(|e| format!("{e:?}"))).collect()
}

fn lcms2_rows(oracle: &Oracle, side: SrgbSide, bpc: Bpc, g: &[Vec<f64>]) -> Result<(Vec<Vec<f64>>, usize), String> {
    let output = match side {
        SrgbSide::Builtin => Space::srgb_builtin(),
        SrgbSide::System => Space::profile(SRGB_SYS),
    };
    let req = Request {
        input: Space::profile(SWOP),
        output,
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        bpc,
        values: g.iter().flatten().map(|x| x * 100.0).collect(),
    };
    let rows = oracle.convert_batch_shaped(&req, 4, 3).map_err(|e| e.to_string())?;
    // transicc prints RGB 0..255 (Pass 5c's harness bug); clamp as Pass 4 does.
    let mut excursions = 0;
    let rows = rows
        .into_iter()
        .map(|r| {
            r.into_iter()
                .map(|v| {
                    let n = v / 255.0;
                    excursions += usize::from(!(0.0..=1.0).contains(&n));
                    n.clamp(0.0, 1.0)
                })
                .collect()
        })
        .collect();
    Ok((rows, excursions))
}

fn per_point(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<f64> {
    a.iter()
        .zip(b)
        .map(|(x, y)| x.iter().zip(y).map(|(p, q)| (p - q).abs()).fold(0.0, f64::max))
        .collect()
}

fn argmax(v: &[f64], g: &[Vec<f64>]) -> Vec<f64> {
    let i = (0..v.len()).max_by(|&a, &b| v[a].total_cmp(&v[b])).unwrap_or(0);
    g[i].clone()
}

fn max_p95(mut v: Vec<f64>) -> (f64, f64) {
    v.sort_by(f64::total_cmp);
    let p95 = v[((v.len() as f64) * 0.95).ceil() as usize - 1];
    (*v.last().unwrap_or(&f64::NAN), p95)
}

fn sub(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    a.iter().zip(b).map(|(x, y)| x.iter().zip(y).map(|(p, q)| p - q).collect()).collect()
}

fn interior(rows: [&[Vec<f64>]; 4], i: usize) -> bool {
    rows.iter().all(|r| r[i].iter().all(|&v| v > 1e-9 && v < 1.0 - 1e-9))
}

/// dv/dR of the sRGB encoding at encoded value `v` (IEC 61966-2-1 piecewise).
fn srgb_slope(v: f64) -> f64 {
    if v <= 0.04045 {
        12.92
    } else {
        let r = ((v + 0.055) / 1.055).powf(2.4);
        1.055 / 2.4 * r.powf(1.0 / 2.4 - 1.0)
    }
}

/// Per component: e_off * (s(v_on) / (s(v_off) * (1 - Ybs)) - 1), the
/// increment a pure interpolation disagreement e_off produces once BPC scales
/// the PCS by 1/(1-Ybs) and moves the operating point along the TRC.
fn model_increment(i_on: &[f64], i_off: &[f64], l_off: &[f64], ybs: f64) -> Vec<f64> {
    (0..i_on.len())
        .map(|c| (i_off[c] - l_off[c]) * (srgb_slope(i_on[c]) / (srgb_slope(i_off[c]) * (1.0 - ybs)) - 1.0))
        .collect()
}

fn end_to_end(oracle: &Oracle, side: SrgbSide) -> Result<EndToEnd, String> {
    let src = load(Path::new(SWOP))?;
    let sys = match side {
        SrgbSide::System => Some(load(Path::new(SRGB_SYS))?),
        SrgbSide::Builtin => None,
    };
    let g = grid();
    let (i_on, i_off) = (icccer_rows(&src, sys.as_ref(), true, &g)?, icccer_rows(&src, sys.as_ref(), false, &g)?);
    let (l_on, e1) = lcms2_rows(oracle, side, Bpc::On, &g)?;
    let (l_off, e2) = lcms2_rows(oracle, side, Bpc::Off, &g)?;
    let (d_on, d_off) = (per_point(&i_on, &l_on), per_point(&i_off, &l_off));
    let d_inc = per_point(&sub(&i_on, &i_off), &sub(&l_on, &l_off));
    let (at_on_max, at_off_max, at_increment_max) = (argmax(&d_on, &g), argmax(&d_off, &g), argmax(&d_inc, &g));
    let (on_max, on_p95) = max_p95(d_on);
    let (off_max, off_p95) = max_p95(d_off);
    let keep: Vec<f64> = (0..g.len())
        .filter(|&i| interior([&i_on, &i_off, &l_on, &l_off], i))
        .map(|i| d_inc[i])
        .collect();
    let ybs = Chain::with_destination(&src, Destination::None, CmmIntent::MediaRelative)
        .and_then(Chain::with_bpc)
        .ok()
        .and_then(|c| c.bpc_disclosure())
        .map_or(0.0, |b| b.source_black().y);
    let inc_signed = sub(&sub(&i_on, &i_off), &sub(&l_on, &l_off));
    let (mut model_max, mut resid_max) = (0.0_f64, 0.0_f64);
    for i in (0..g.len()).filter(|&i| interior([&i_on, &i_off, &l_on, &l_off], i)) {
        let m = model_increment(&i_on[i], &i_off[i], &l_off[i], ybs);
        for c in 0..m.len() {
            model_max = model_max.max(m[c].abs());
            resid_max = resid_max.max((inc_signed[i][c] - m[c]).abs());
        }
    }
    let unclamped_points = keep.len();
    let (inc_u_max, inc_u_p95) = max_p95(keep);
    let (inc_max, inc_p95) = max_p95(d_inc);
    Ok(EndToEnd {
        side,
        points: g.len(),
        on_max,
        on_p95,
        off_max,
        off_p95,
        icccer_bpc_effect_max: max_p95(per_point(&i_on, &i_off)).0,
        lcms2_bpc_effect_max: max_p95(per_point(&l_on, &l_off)).0,
        increment_max: inc_max,
        increment_p95: inc_p95,
        increment_unclamped_max: inc_u_max,
        increment_unclamped_p95: inc_u_p95,
        unclamped_points,
        model_increment_max: model_max,
        model_residual_max: resid_max,
        no_bpc_alt_max: max_p95(per_point(&i_off, &l_on)).0,
        lcms2_excursions: e1 + e2,
        at_on_max,
        at_off_max,
        at_increment_max,
    })
}

/// Everything Pass 5d measured.
#[derive(Debug, Clone)]
pub struct Analysis {
    pub arms: Vec<BlackArm>,
    pub diag: Diagnostics,
    pub e2e: Vec<EndToEnd>,
}

/// Measure. `Err` carries a skip reason (absent profile, absent probe).
pub fn analyse(oracle: &Oracle) -> Result<Analysis, String> {
    if !Path::new(SWOP).is_file() {
        return Err(format!("profile not present on this machine: {SWOP} (LEGAL.md §3 (c))"));
    }
    if !probe_exe().is_file() {
        return Err(format!("{} absent: run probes/build-probe.ps1", probe_exe().display()));
    }
    let arms = ARMS.iter().map(|&(n, i)| measure_arm(n, i)).collect::<Result<Vec<_>, _>>()?;
    let mut e2e = vec![end_to_end(oracle, SrgbSide::Builtin)?];
    if Path::new(SRGB_SYS).is_file() {
        e2e.push(end_to_end(oracle, SrgbSide::System)?);
    }
    Ok(Analysis { arms, diag: diagnostics()?, e2e })
}

pub use crate::pass5d_records::records;

/// Run Pass 5d: the analysis plus its records, or skip rows naming why.
pub fn run(oracle: &Oracle) -> (Option<Analysis>, Vec<Record>) {
    match analyse(oracle) {
        Ok(a) => {
            let r = records(&a);
            (Some(a), r)
        }
        Err(reason) => (
            None,
            vec![Record::skipped(
                "pass5d/all",
                Kind::CrossCheck,
                Metric::AbsMaxComponent,
                Tolerance::new(f64::INFINITY, "skipped; see reason"),
                SRC,
                reason,
            )],
        ),
    }
}

pub(crate) const SOURCE: &str = SRC;
