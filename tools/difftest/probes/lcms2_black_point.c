/* lcms2_black_point.c — print the pinned lcms2's cmsDetectBlackPoint result.
 *
 * transicc cannot print a black point; this probe calls the function itself,
 * so the source-black cross-check (README §29) has no recovery step in it.
 * Links the pinned static lcms2.lib only; never part of a shipping crate.
 *
 *   lcms2_black_point <profile.icc> <intent 0|1|2>
 *   lcms2_black_point --lab <profile.icc> <intent> c m y k   (inks 0..1)
 *   lcms2_black_point --b2a <profile.icc> <intent> L a b
 *
 * --lab is the device->Lab leg exactly as BlackPointAsDarkerColorant builds it
 * (Lab2 output, NOOPTIMIZE|NOCACHE). --b2a is the Lab->device leg exactly as
 * CreateRoundtripXForm builds it (Lab4 input), so on a v4 profile at
 * perceptual/saturation it INCLUDES lcms2's forced BPC (cmscnvrt.c L1132).
 *
 * One line on stdout, tab-separated key=value:
 *   ok=<0|1> X= Y= Z= L= a= b=   (Lab relative to D50, cmsXYZ2Lab(NULL,..))
 *   ver=0x........ class=.... space=.... matrix_shaper=<0|1>
 *
 * The `branch=` field is this probe's READING of cmssamp.c L262/L314 at the
 * pin (21c582a), not something lcms2 reports. Build: README §29.
 */
#include <stdio.h>
#include <stdlib.h>
#include "lcms2.h"

static void sig4(cmsUInt32Number s, char out[5]) {
    out[0] = (char)(s >> 24); out[1] = (char)(s >> 16);
    out[2] = (char)(s >> 8);  out[3] = (char)s; out[4] = 0;
}

static int is_ink(cmsColorSpaceSignature c) {
    return c == cmsSigCmykData || c == cmsSigCmyData ||
           (c >= cmsSigMCH1Data && c <= cmsSigMCHFData) ||
           (c >= cmsSig1colorData && c <= cmsSig15colorData);
}

static int leg(int argc, char **argv) {
    int lab = argv[1][2] == 'l';
    cmsHPROFILE h = cmsOpenProfileFromFile(argv[2], "r");
    if (!h) return 2;
    cmsUInt32Number intent = (cmsUInt32Number)atoi(argv[3]);
    cmsHPROFILE pcs = lab ? cmsCreateLab2Profile(NULL) : cmsCreateLab4Profile(NULL);
    cmsUInt32Number dev = cmsFormatterForColorspaceOfProfile(h, 0, TRUE);
    int n = (int)T_CHANNELS(dev);
    cmsHTRANSFORM x = lab
        ? cmsCreateTransform(h, dev, pcs, TYPE_Lab_DBL, intent, cmsFLAGS_NOOPTIMIZE | cmsFLAGS_NOCACHE)
        : cmsCreateTransform(pcs, TYPE_Lab_DBL, h, dev, intent, cmsFLAGS_NOOPTIMIZE | cmsFLAGS_NOCACHE);
    if (!x) return 3;
    double in[16] = {0}, out[16] = {0};
    int nin = lab ? n : 3, nout = lab ? 3 : n;
    if (argc != 4 + nin) return 2;
    /* CMYK only: lcms2's float ink is 0..100, this interface is 0..1. */
    for (int i = 0; i < nin; i++) in[i] = atof(argv[4 + i]) * (lab ? 100.0 : 1.0);
    cmsDoTransform(x, in, out, 1);
    for (int i = 0; i < nout; i++) printf("%s%.9f", i ? "\t" : "", out[i] / (lab ? 1.0 : 100.0));
    printf("\n");
    return 0;
}

int main(int argc, char **argv) {
    if (argc >= 4 && argv[1][0] == '-' && argv[1][1] == '-') return leg(argc, argv);
    if (argc != 3) { fprintf(stderr, "usage: %s profile intent\n", argv[0]); return 2; }
    cmsHPROFILE h = cmsOpenProfileFromFile(argv[1], "r");
    if (!h) { fprintf(stderr, "cannot open %s\n", argv[1]); return 2; }
    cmsUInt32Number intent = (cmsUInt32Number)atoi(argv[2]);

    cmsCIEXYZ bp; cmsCIELab lab;
    cmsBool ok = cmsDetectBlackPoint(&bp, h, intent, 0);
    cmsXYZ2Lab(NULL, &lab, &bp);

    cmsUInt32Number ver = cmsGetEncodedICCversion(h);
    char cls[5], spc[5];
    sig4(cmsGetDeviceClass(h), cls);
    sig4(cmsGetColorSpace(h), spc);
    int ms = cmsIsMatrixShaper(h) ? 1 : 0;

    const char *branch;
    if (ver >= 0x4000000 && (intent == 0 || intent == 2))
        branch = ms ? "DarkerColorant(rel)" : "FixedPerceptualBlack";
    else if (intent == 1 && cmsGetDeviceClass(h) == cmsSigOutputClass && is_ink(cmsGetColorSpace(h)))
        branch = "UsingPerceptualBlack";
    else
        branch = "DarkerColorant(intent)";

    printf("ok=%d\tX=%.9f\tY=%.9f\tZ=%.9f\tL=%.9f\ta=%.9f\tb=%.9f\tver=0x%08x\tclass=%s\tspace=%s\tmatrix_shaper=%d\tbranch=%s\n",
           ok ? 1 : 0, bp.X, bp.Y, bp.Z, lab.L, lab.a, lab.b, ver, cls, spc, ms, branch);
    cmsCloseProfile(h);
    return 0;
}
