//! Pass 5d's records and tolerances. Measurement: `pass5d.rs`. Every
//! tolerance's derivation: `docs/TOLERANCES.md` §3.5.10.

use crate::pass5d::{Analysis, BlackArm, EndToEnd, de76, lab_abs_max, SOURCE};
use crate::{Intent, Kind, Metric, Record, SepUnits, Separation, Tolerance};

/// Two u16 codes of a device component (0..1).
pub const LEG_U16: Tolerance = Tolerance::new(
    2.0 / 65535.0,
    "both engines evaluate the same B2A0 table at the same PCS point; lcms2 rounds the CLUT \
     input to u16 (EvaluateCLUTfloatIn16) and stores u16 table values, so near one point the two \
     differ by quantisation only: two LSBs. Any other LocalBlack (Lab 0 vs PERCEPTUAL_BLACK on the \
     v4 fixture) moves K by >= 3e-2, three orders above",
);

/// lcms2 reproducing its own result from a supplied input.
pub const ORACLE_IDENTITY: Tolerance = Tolerance::new(
    1e-6,
    "the same lcms2 code on device inputs that round to the same u16 codes returns the same \
     number; the probe prints 9 decimals, so 5e-10 is the floor. Exceeding 1e-6 means the \
     decomposition is not the whole mechanism",
);

/// The v4 perceptual constant, both sides.
pub const CONSTANT_IDENTITY: Tolerance = Tolerance::new(
    1e-8,
    "both sides return the same published constant (ISO/CD 18619 A.2 via cmsPERCEPTUAL_BLACK); \
     the probe's 9-decimal print floors the comparison at 5e-10. Any other branch moves L* by \
     >= 0.78",
);

/// A black arm where the engines run different, documented algorithms.
pub const POLICY_REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED: icccer and lcms2 run DIFFERENT estimators on this arm (branch named in the detail); \
     no normative BPC text decides the source black (A27/A42), and where ISO/CD 18619 4.2.3 does \
     speak, icccer follows it (README §29.3). A policy difference has no arithmetic bound",
);

/// SWOP relative: the same estimator; the residue is one A2B1 evaluation.
pub const INTERP_REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED: both engines run 4.2.3 here and their LocalBlacks agree to two u16 codes      (mechanism rows), so the difference is one A2B1 evaluation's CLUT interpolation (Pass 4 M4,      graded there). The mechanism rows carry the gate; a bound here would re-grade Pass 4",
);

/// End to end, BPC on, icccer vs lcms2.
pub const E2E_BPC_ON: Tolerance = Tolerance::new(
    2e-2,
    "Pass 4's DEVICE_CROSSCHECK for the same SWOP 9^4 tables (interpolation-method envelope, no \
     lcms2 output), transported through BPC: the map scales PCS by 1/(1-Ybs)=1.023 and moves the \
     operating point darker, where the sRGB encode slope is <= 12.92 against >= 4.07 at the \
     unscaled black, so the off-disagreement grows by at most 3.25x; plus 12.92 x the black-point \
     difference's dY' <= 3.5e-5 = 4.5e-4. 3.25 x 4.70e-3 (this grid's BPC-off max) + 4.5e-4 = \
     1.57e-2 <= 2e-2. GRID-DEPENDENT, arithmetic agreement, NOT perceptual",
);

/// The first-order model's residual on the unclamped population.
pub const MODEL_RESIDUAL: Tolerance = Tolerance::new(
    1e-3,
    "what the BPC increment owes the estimator rather than interpolation: 12.92 x dY' <= 4.5e-4 \
     from the 0.017 L* black difference, plus the model's linearisation error 1/2|f''|dR^2 ~ \
     1.7e-4 per side at the lowest gamma-segment point = 7.9e-4; 1e-3 admits a tabulated sRGB \
     TRC where the model uses the IEC formula. Derived AFTER the measurement, closed form, no \
     residual used",
);

/// The magnitude of BPC itself: reported, not gated.
pub const BPC_EFFECT_REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED: how far BPC moves the output is a policy quantity with no normative text \
     (A27/A42); its agreement with lcms2 is the information, not a gate",
);

/// ISO/CD 18619 4.2.3 structure: neutral, L* <= 50 (A41 constant excepted).
pub const STRUCTURE: Tolerance = Tolerance::new(
    0.0,
    "4.2.3 neutralises and clips at L* 50 unconditionally; a non-neutral or >50 result outside \
     the v4-perceptual constant is a defect, and a count has no rounding",
);

fn intent_tag(i: Intent) -> &'static str {
    match i {
        Intent::Perceptual => "perceptual",
        Intent::RelativeColorimetric => "relative",
        Intent::Saturation => "saturation",
        Intent::AbsoluteColorimetric => "absolute",
    }
}

fn max_abs_diff(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f64::max)
}

fn black_row(arm: &BlackArm) -> Record {
    let id = format!("pass5d/black/{}/{}", arm.name, intent_tag(arm.intent));
    let tol = match (arm.name, arm.intent) {
        ("mab", Intent::Perceptual) => CONSTANT_IDENTITY,
        ("swop", Intent::RelativeColorimetric) => INTERP_REPORTED,
        _ => POLICY_REPORTED,
    };
    let (i, l) = (arm.icccer, arm.lcms2.lab);
    let detail = format!(
        "icccer Lab({:.6}, {:.6}, {:.6}) vs lcms2 cmsDetectBlackPoint Lab({:.6}, {:.6}, {:.6}), \
         dE76 {:.6}, lcms2 branch {} (probe's reading of cmssamp.c)",
        i.l, i.a, i.b, l.l, l.a, l.b, de76(i, l), arm.lcms2.branch
    );
    Record::graded(id, Kind::CrossCheck, Metric::AbsMaxComponent, tol, lab_abs_max(i, l), SOURCE, detail)
}

fn structure_row(a: &Analysis) -> Record {
    let bad = a
        .arms
        .iter()
        .filter(|r| !(r.name == "mab" && r.intent == Intent::Perceptual))
        .filter(|r| r.icccer.a != 0.0 || r.icccer.b != 0.0 || r.icccer.l > 50.0 || r.icccer.l < 0.0)
        .count();
    Record::graded(
        "pass5d/black/icccer-4.2.3-structure",
        Kind::DerivedExpectation,
        Metric::IndicatorCount,
        STRUCTURE,
        bad as f64,
        SOURCE,
        format!("{} arms outside the v4-perceptual constant, each neutral with 0 <= L* <= 50", a.arms.len() - 1),
    )
}

fn mechanism_rows(a: &Analysis) -> Vec<Record> {
    let d = &a.diag;
    let lab_l = |name: &str, intent| a.arms.iter().find(|r| r.name == name && r.intent == intent).map_or(f64::NAN, |r| r.lcms2.lab.l);
    let swop_rel = lab_l("swop", Intent::RelativeColorimetric);
    let mab_rel = lab_l("mab", Intent::RelativeColorimetric);
    let lit = max_abs_diff(&d.mab_lb_icccer, &d.mab_lb_lcms2);
    vec![
        Record::graded(
            "pass5d/mechanism/swop/localblack-b2a0-lab0",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            LEG_U16,
            max_abs_diff(&d.swop_lb_icccer, &d.swop_lb_lcms2),
            SOURCE,
            format!("icccer B2A0(Lab 0) {:?} vs lcms2's estimator leg {:?}", d.swop_lb_icccer, d.swop_lb_lcms2),
        ),
        Record::graded(
            "pass5d/mechanism/swop/relative-is-all-a2b1",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            ORACLE_IDENTITY,
            (d.swop_lcms2_a2b1_of_icccer_lb[0] - swop_rel).abs(),
            SOURCE,
            format!(
                "lcms2 A2B1 of icccer's LocalBlack gives L* {:.9}, lcms2's own black {:.9}: the \
                 0.017 L* arm difference is the A2B1 interpolation (Pass 4 M4), not the estimator",
                d.swop_lcms2_a2b1_of_icccer_lb[0], swop_rel
            ),
        ),
        Record::graded(
            "pass5d/mechanism/swop/perceptual-is-400pct-ink",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            ORACLE_IDENTITY,
            d.swop_a2b0_400[0].abs().max(d.swop_a2b2_400[0].abs()),
            SOURCE,
            format!(
                "lcms2 A2B0/A2B2 of CMYK 400% give L* {:.9}/{:.9} = its black at those intents: \
                 BlackPointAsDarkerColorant, not 4.2.3's B2A0(Lab 0)",
                d.swop_a2b0_400[0], d.swop_a2b2_400[0]
            ),
        ),
        Record::graded(
            "pass5d/mechanism/mab/lcms2-leg-is-b2a0-of-perceptual-black",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            LEG_U16,
            max_abs_diff(&d.mab_lb_icccer_at_prm_black, &d.mab_lb_lcms2),
            SOURCE,
            format!(
                "lcms2's leg {:?} = icccer B2A0(PERCEPTUAL_BLACK) {:?}: lcms2 forces BPC inside its \
                 own round trip (cmscnvrt.c L1132); ISO 4.2.3 says B2A0(Lab 0) = {:?}",
                d.mab_lb_lcms2, d.mab_lb_icccer_at_prm_black, d.mab_lb_icccer
            ),
        )
        .with_separation(Separation::against(
            "LocalBlack = B2A0(Lab 0), the literal 4.2.3 reading",
            lit,
            max_abs_diff(&d.mab_lb_icccer_at_prm_black, &d.mab_lb_lcms2),
            SepUnits::SameAsMetric,
        )),
        Record::graded(
            "pass5d/mechanism/mab/relative-is-forced-bpc",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            ORACLE_IDENTITY,
            (d.mab_lcms2_a2b1_of_prm_lb[0] - mab_rel).abs(),
            SOURCE,
            format!(
                "lcms2 A2B1 of B2A0(PERCEPTUAL_BLACK) gives L* {:.9} (a,b {:.3}, {:.3} before \
                 neutralising), lcms2's own black {:.9}: the whole 3.137 dE76 is the forced BPC",
                d.mab_lcms2_a2b1_of_prm_lb[0], d.mab_lcms2_a2b1_of_prm_lb[1], d.mab_lcms2_a2b1_of_prm_lb[2], mab_rel
            ),
        ),
    ]
}

fn e2e_rows(e: &EndToEnd) -> Vec<Record> {
    let p = format!("pass5d/e2e/swop-to-{}", e.side.tag());
    vec![
        Record::graded(
            format!("{p}/bpc-on"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            E2E_BPC_ON,
            e.on_max,
            SOURCE,
            format!(
                "media-relative, {} CMYK points: BPC on max {:.4e} at {:?}, p95 {:.4e}; BPC off max \
                 {:.4e} at {:?}, p95 {:.4e}; lcms2 components outside [0,1] clamped: {}",
                e.points, e.on_max, e.at_on_max, e.on_p95, e.off_max, e.at_off_max, e.off_p95, e.lcms2_excursions
            ),
        )
        .with_separation(Separation::against(
            "icccer drops BPC (icccer off vs lcms2 on)",
            e.no_bpc_alt_max,
            e.on_max,
            SepUnits::SameAsMetric,
        )),
        Record::graded(
            format!("{p}/increment-model-residual"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            MODEL_RESIDUAL,
            e.model_residual_max,
            SOURCE,
            format!(
                "BPC increment disagreement over {} unclamped points: max {:.4e}, p95 {:.4e} (full \
                 grid {:.4e}, clamp-bounded); the BPC-off disagreement re-seen through the TRC \
                 slope predicts {:.4e}, leaving {:.4e}",
                e.unclamped_points,
                e.increment_unclamped_max,
                e.increment_unclamped_p95,
                e.increment_max,
                e.model_increment_max,
                e.model_residual_max
            ),
        ),
        Record::graded(
            format!("{p}/bpc-effect"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            BPC_EFFECT_REPORTED,
            (e.icccer_bpc_effect_max - e.lcms2_bpc_effect_max).abs(),
            SOURCE,
            format!(
                "BPC moves icccer's output by up to {:.6}, lcms2's by {:.6}",
                e.icccer_bpc_effect_max, e.lcms2_bpc_effect_max
            ),
        ),
    ]
}

pub fn records(a: &Analysis) -> Vec<Record> {
    let mut v: Vec<Record> = a.arms.iter().map(black_row).collect();
    v.push(structure_row(a));
    v.extend(mechanism_rows(a));
    for e in &a.e2e {
        v.extend(e2e_rows(e));
    }
    v
}
