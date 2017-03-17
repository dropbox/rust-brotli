extern {
    fn BrotliAllocate(
        m : *mut MemoryManager, n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn BrotliFree(
        m : *mut MemoryManager, p : *mut ::std::os::raw::c_void
    );
    fn BrotliHistogramBitCostDistanceCommand(
        histogram : *const HistogramCommand,
        candidate : *const HistogramCommand
    ) -> f64;
    fn BrotliHistogramBitCostDistanceDistance(
        histogram : *const HistogramDistance,
        candidate : *const HistogramDistance
    ) -> f64;
    fn BrotliHistogramBitCostDistanceLiteral(
        histogram : *const HistogramLiteral,
        candidate : *const HistogramLiteral
    ) -> f64;
    fn BrotliHistogramCombineCommand(
        out : *mut HistogramCommand,
        cluster_size : *mut u32,
        symbols : *mut u32,
        clusters : *mut u32,
        pairs : *mut HistogramPair,
        num_clusters : usize,
        symbols_size : usize,
        max_clusters : usize,
        max_num_pairs : usize
    ) -> usize;
    fn BrotliHistogramCombineDistance(
        out : *mut HistogramDistance,
        cluster_size : *mut u32,
        symbols : *mut u32,
        clusters : *mut u32,
        pairs : *mut HistogramPair,
        num_clusters : usize,
        symbols_size : usize,
        max_clusters : usize,
        max_num_pairs : usize
    ) -> usize;
    fn BrotliHistogramCombineLiteral(
        out : *mut HistogramLiteral,
        cluster_size : *mut u32,
        symbols : *mut u32,
        clusters : *mut u32,
        pairs : *mut HistogramPair,
        num_clusters : usize,
        symbols_size : usize,
        max_clusters : usize,
        max_num_pairs : usize
    ) -> usize;
    fn BrotliPopulationCostCommand(
        arg1 : *const HistogramCommand
    ) -> f64;
    fn BrotliPopulationCostDistance(
        arg1 : *const HistogramDistance
    ) -> f64;
    fn BrotliPopulationCostLiteral(
        arg1 : *const HistogramLiteral
    ) -> f64;
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn log2(__x : f64) -> f64;
    fn memcpy(
        __dest : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __s : *mut ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
}

static mut kLog2Table
    : [f32; 256]
    = [   0.0000000000000000f32,
          0.0000000000000000f32,
          1.0000000000000000f32,
          1.5849625007211563f32,
          2.0000000000000000f32,
          2.3219280948873622f32,
          2.5849625007211561f32,
          2.8073549220576042f32,
          3.0000000000000000f32,
          3.1699250014423126f32,
          3.3219280948873626f32,
          3.4594316186372978f32,
          3.5849625007211565f32,
          3.7004397181410922f32,
          3.8073549220576037f32,
          3.9068905956085187f32,
          4.0000000000000000f32,
          4.0874628412503400f32,
          4.1699250014423122f32,
          4.2479275134435852f32,
          4.3219280948873626f32,
          4.3923174227787607f32,
          4.4594316186372973f32,
          4.5235619560570131f32,
          4.5849625007211570f32,
          4.6438561897747244f32,
          4.7004397181410926f32,
          4.7548875021634691f32,
          4.8073549220576037f32,
          4.8579809951275728f32,
          4.9068905956085187f32,
          4.9541963103868758f32,
          5.0000000000000000f32,
          5.0443941193584534f32,
          5.0874628412503400f32,
          5.1292830169449664f32,
          5.1699250014423122f32,
          5.2094533656289501f32,
          5.2479275134435852f32,
          5.2854022188622487f32,
          5.3219280948873626f32,
          5.3575520046180838f32,
          5.3923174227787607f32,
          5.4262647547020979f32,
          5.4594316186372973f32,
          5.4918530963296748f32,
          5.5235619560570131f32,
          5.5545888516776376f32,
          5.5849625007211570f32,
          5.6147098441152083f32,
          5.6438561897747244f32,
          5.6724253419714961f32,
          5.7004397181410926f32,
          5.7279204545631996f32,
          5.7548875021634691f32,
          5.7813597135246599f32,
          5.8073549220576046f32,
          5.8328900141647422f32,
          5.8579809951275719f32,
          5.8826430493618416f32,
          5.9068905956085187f32,
          5.9307373375628867f32,
          5.9541963103868758f32,
          5.9772799234999168f32,
          6.0000000000000000f32,
          6.0223678130284544f32,
          6.0443941193584534f32,
          6.0660891904577721f32,
          6.0874628412503400f32,
          6.1085244567781700f32,
          6.1292830169449672f32,
          6.1497471195046822f32,
          6.1699250014423122f32,
          6.1898245588800176f32,
          6.2094533656289510f32,
          6.2288186904958804f32,
          6.2479275134435861f32,
          6.2667865406949019f32,
          6.2854022188622487f32,
          6.3037807481771031f32,
          6.3219280948873617f32,
          6.3398500028846252f32,
          6.3575520046180847f32,
          6.3750394313469254f32,
          6.3923174227787598f32,
          6.4093909361377026f32,
          6.4262647547020979f32,
          6.4429434958487288f32,
          6.4594316186372982f32,
          6.4757334309663976f32,
          6.4918530963296748f32,
          6.5077946401986964f32,
          6.5235619560570131f32,
          6.5391588111080319f32,
          6.5545888516776376f32,
          6.5698556083309478f32,
          6.5849625007211561f32,
          6.5999128421871278f32,
          6.6147098441152092f32,
          6.6293566200796095f32,
          6.6438561897747253f32,
          6.6582114827517955f32,
          6.6724253419714952f32,
          6.6865005271832185f32,
          6.7004397181410917f32,
          6.7142455176661224f32,
          6.7279204545631988f32,
          6.7414669864011465f32,
          6.7548875021634691f32,
          6.7681843247769260f32,
          6.7813597135246599f32,
          6.7944158663501062f32,
          6.8073549220576037f32,
          6.8201789624151887f32,
          6.8328900141647422f32,
          6.8454900509443757f32,
          6.8579809951275719f32,
          6.8703647195834048f32,
          6.8826430493618416f32,
          6.8948177633079437f32,
          6.9068905956085187f32,
          6.9188632372745955f32,
          6.9307373375628867f32,
          6.9425145053392399f32,
          6.9541963103868758f32,
          6.9657842846620879f32,
          6.9772799234999168f32,
          6.9886846867721664f32,
          7.0000000000000000f32,
          7.0112272554232540f32,
          7.0223678130284544f32,
          7.0334230015374501f32,
          7.0443941193584534f32,
          7.0552824355011898f32,
          7.0660891904577721f32,
          7.0768155970508317f32,
          7.0874628412503400f32,
          7.0980320829605272f32,
          7.1085244567781700f32,
          7.1189410727235076f32,
          7.1292830169449664f32,
          7.1395513523987937f32,
          7.1497471195046822f32,
          7.1598713367783891f32,
          7.1699250014423130f32,
          7.1799090900149345f32,
          7.1898245588800176f32,
          7.1996723448363644f32,
          7.2094533656289492f32,
          7.2191685204621621f32,
          7.2288186904958804f32,
          7.2384047393250794f32,
          7.2479275134435861f32,
          7.2573878426926521f32,
          7.2667865406949019f32,
          7.2761244052742384f32,
          7.2854022188622487f32,
          7.2946207488916270f32,
          7.3037807481771031f32,
          7.3128829552843557f32,
          7.3219280948873617f32,
          7.3309168781146177f32,
          7.3398500028846243f32,
          7.3487281542310781f32,
          7.3575520046180847f32,
          7.3663222142458151f32,
          7.3750394313469254f32,
          7.3837042924740528f32,
          7.3923174227787607f32,
          7.4008794362821844f32,
          7.4093909361377026f32,
          7.4178525148858991f32,
          7.4262647547020979f32,
          7.4346282276367255f32,
          7.4429434958487288f32,
          7.4512111118323299f32,
          7.4594316186372973f32,
          7.4676055500829976f32,
          7.4757334309663976f32,
          7.4838157772642564f32,
          7.4918530963296748f32,
          7.4998458870832057f32,
          7.5077946401986964f32,
          7.5156998382840436f32,
          7.5235619560570131f32,
          7.5313814605163119f32,
          7.5391588111080319f32,
          7.5468944598876373f32,
          7.5545888516776376f32,
          7.5622424242210728f32,
          7.5698556083309478f32,
          7.5774288280357487f32,
          7.5849625007211561f32,
          7.5924570372680806f32,
          7.5999128421871278f32,
          7.6073303137496113f32,
          7.6147098441152075f32,
          7.6220518194563764f32,
          7.6293566200796095f32,
          7.6366246205436488f32,
          7.6438561897747244f32,
          7.6510516911789290f32,
          7.6582114827517955f32,
          7.6653359171851765f32,
          7.6724253419714952f32,
          7.6794800995054464f32,
          7.6865005271832185f32,
          7.6934869574993252f32,
          7.7004397181410926f32,
          7.7073591320808825f32,
          7.7142455176661224f32,
          7.7210991887071856f32,
          7.7279204545631996f32,
          7.7347096202258392f32,
          7.7414669864011465f32,
          7.7481928495894596f32,
          7.7548875021634691f32,
          7.7615512324444795f32,
          7.7681843247769260f32,
          7.7747870596011737f32,
          7.7813597135246608f32,
          7.7879025593914317f32,
          7.7944158663501062f32,
          7.8008998999203047f32,
          7.8073549220576037f32,
          7.8137811912170374f32,
          7.8201789624151887f32,
          7.8265484872909159f32,
          7.8328900141647422f32,
          7.8392037880969445f32,
          7.8454900509443757f32,
          7.8517490414160571f32,
          7.8579809951275719f32,
          7.8641861446542798f32,
          7.8703647195834048f32,
          7.8765169465650002f32,
          7.8826430493618425f32,
          7.8887432488982601f32,
          7.8948177633079446f32,
          7.9008668079807496f32,
          7.9068905956085187f32,
          7.9128893362299619f32,
          7.9188632372745955f32,
          7.9248125036057813f32,
          7.9307373375628867f32,
          7.9366379390025719f32,
          7.9425145053392399f32,
          7.9483672315846778f32,
          7.9541963103868758f32,
          7.9600019320680806f32,
          7.9657842846620870f32,
          7.9715435539507720f32,
          7.9772799234999168f32,
          7.9829935746943104f32,
          7.9886846867721664f32,
          7.9943534368588578f32
      ];

static mut kInsBase
    : [u32; 24]
    = [   0i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          8i32 as (u32),
          10i32 as (u32),
          14i32 as (u32),
          18i32 as (u32),
          26i32 as (u32),
          34i32 as (u32),
          50i32 as (u32),
          66i32 as (u32),
          98i32 as (u32),
          130i32 as (u32),
          194i32 as (u32),
          322i32 as (u32),
          578i32 as (u32),
          1090i32 as (u32),
          2114i32 as (u32),
          6210i32 as (u32),
          22594i32 as (u32)
      ];

static mut kInsExtra
    : [u32; 24]
    = [   0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          12i32 as (u32),
          14i32 as (u32),
          24i32 as (u32)
      ];

static mut kCopyBase
    : [u32; 24]
    = [   2i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          12i32 as (u32),
          14i32 as (u32),
          18i32 as (u32),
          22i32 as (u32),
          30i32 as (u32),
          38i32 as (u32),
          54i32 as (u32),
          70i32 as (u32),
          102i32 as (u32),
          134i32 as (u32),
          198i32 as (u32),
          326i32 as (u32),
          582i32 as (u32),
          1094i32 as (u32),
          2118i32 as (u32)
      ];

static mut kCopyExtra
    : [u32; 24]
    = [   0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          24i32 as (u32)
      ];

static kBrotliMinWindowBits : i32 = 10i32;

static kBrotliMaxWindowBits : i32 = 24i32;

static mut kUTF8ContextLookup
    : [u8; 512]
    = [   0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          4i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          8i32 as (u8),
          12i32 as (u8),
          16i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          20i32 as (u8),
          12i32 as (u8),
          16i32 as (u8),
          24i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          32i32 as (u8),
          12i32 as (u8),
          36i32 as (u8),
          12i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          32i32 as (u8),
          32i32 as (u8),
          24i32 as (u8),
          40i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          24i32 as (u8),
          12i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          24i32 as (u8),
          12i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8)
      ];

static mut kSigned3BitContextLookup
    : [u8; 256]
    = [   0i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          7i32 as (u8)
      ];

static kMaxLiteralHistograms : usize = 100i32 as (usize);

static kMaxCommandHistograms : usize = 50i32 as (usize);

static kLiteralBlockSwitchCost : f64 = 28.1f64;

static kCommandBlockSwitchCost : f64 = 13.5f64;

static kDistanceBlockSwitchCost : f64 = 14.6f64;

static kLiteralStrideLength : usize = 70i32 as (usize);

static kCommandStrideLength : usize = 40i32 as (usize);

static kSymbolsPerLiteralHistogram : usize = 544i32 as (usize);

static kSymbolsPerCommandHistogram : usize = 530i32 as (usize);

static kSymbolsPerDistanceHistogram : usize = 544i32 as (usize);

static kMinLengthForBlockSplitting : usize = 128i32 as (usize);

static kIterMulForRefining : usize = 2i32 as (usize);

static kMinItersForRefining : usize = 100i32 as (usize);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockSplit {
    pub num_types : usize,
    pub num_blocks : usize,
    pub types : *mut u8,
    pub lengths : *mut u32,
    pub types_alloc_size : usize,
    pub lengths_alloc_size : usize,
}

#[no_mangle]
pub unsafe extern fn BrotliInitBlockSplit(mut self : *mut BlockSplit) {
    (*self).num_types = 0i32 as (usize);
    (*self).num_blocks = 0i32 as (usize);
    (*self).types = 0i32 as (*mut u8);
    (*self).lengths = 0i32 as (*mut u32);
    (*self).types_alloc_size = 0i32 as (usize);
    (*self).lengths_alloc_size = 0i32 as (usize);
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
    pub opaque : *mut ::std::os::raw::c_void,
}

#[no_mangle]
pub unsafe extern fn BrotliDestroyBlockSplit(
    mut m : *mut MemoryManager, mut self : *mut BlockSplit
) {
    BrotliFree(m,(*self).types as (*mut ::std::os::raw::c_void));
    (*self).types = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    BrotliFree(m,(*self).lengths as (*mut ::std::os::raw::c_void));
    (*self).lengths = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Command {
    pub insert_len_ : u32,
    pub copy_len_ : u32,
    pub dist_extra_ : u32,
    pub cmd_prefix_ : u16,
    pub dist_prefix_ : u16,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum BrotliEncoderMode {
    BROTLI_MODE_GENERIC = 0i32,
    BROTLI_MODE_TEXT = 1i32,
    BROTLI_MODE_FONT = 2i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliHasherParams {
    pub type_ : i32,
    pub bucket_bits : i32,
    pub block_bits : i32,
    pub hash_len : i32,
    pub num_last_distances_to_check : i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliEncoderParams {
    pub mode : BrotliEncoderMode,
    pub quality : i32,
    pub lgwin : i32,
    pub lgblock : i32,
    pub size_hint : usize,
    pub disable_literal_context_modeling : i32,
    pub hasher : BrotliHasherParams,
}

unsafe extern fn CountLiterals(
    mut cmds : *const Command, num_commands : usize
) -> usize {
    let mut total_length : usize = 0i32 as (usize);
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < num_commands {
            total_length = total_length.wrapping_add(
                               (*cmds.offset(i as (isize))).insert_len_ as (usize)
                           );
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    total_length
}

unsafe extern fn CommandCopyLen(mut self : *const Command) -> u32 {
    (*self).copy_len_ & 0xffffffi32 as (u32)
}

unsafe extern fn CopyLiteralsToByteArray(
    mut cmds : *const Command,
    num_commands : usize,
    mut data : *const u8,
    offset : usize,
    mask : usize,
    mut literals : *mut u8
) {
    let mut pos : usize = 0i32 as (usize);
    let mut from_pos : usize = offset & mask;
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < num_commands {
            let mut insert_len
                : usize
                = (*cmds.offset(i as (isize))).insert_len_ as (usize);
            if from_pos.wrapping_add(insert_len) > mask {
                let mut head_size
                    : usize
                    = mask.wrapping_add(1i32 as (usize)).wrapping_sub(from_pos);
                memcpy(
                    literals.offset(pos as (isize)) as (*mut ::std::os::raw::c_void),
                    data.offset(
                        from_pos as (isize)
                    ) as (*const ::std::os::raw::c_void),
                    head_size
                );
                from_pos = 0i32 as (usize);
                pos = pos.wrapping_add(head_size);
                insert_len = insert_len.wrapping_sub(head_size);
            }
            if insert_len > 0i32 as (usize) {
                memcpy(
                    literals.offset(pos as (isize)) as (*mut ::std::os::raw::c_void),
                    data.offset(
                        from_pos as (isize)
                    ) as (*const ::std::os::raw::c_void),
                    insert_len
                );
                pos = pos.wrapping_add(insert_len);
            }
            from_pos = from_pos.wrapping_add(insert_len).wrapping_add(
                           CommandCopyLen(
                               &*cmds.offset(i as (isize)) as (*const Command)
                           ) as (usize)
                       ) & mask;
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn HistogramDataSizeLiteral() -> usize {
    256i32 as (usize)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_ : [u32; 256],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

unsafe extern fn HistogramClearLiteral(
    mut self : *mut HistogramLiteral
) {
    memset(
        (*self).data_.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 256]>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn ClearHistogramsLiteral(
    mut array : *mut HistogramLiteral, mut length : usize
) {
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < length {
            HistogramClearLiteral(array.offset(i as (isize)));
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn MyRand(mut seed : *mut u32) -> u32 {
    *seed = (*seed).wrapping_mul(16807u32);
    if *seed == 0i32 as (u32) {
        *seed = 1i32 as (u32);
    }
    *seed
}

unsafe extern fn HistogramAddVectorLiteral(
    mut self : *mut HistogramLiteral, mut p : *const u8, mut n : usize
) {
    (*self).total_count_ = (*self).total_count_.wrapping_add(n);
    n = n.wrapping_add(1i32 as (usize));
    'loop1: loop {
        if {
               n = n.wrapping_sub(1 as (usize));
               n
           } != 0 {
            {
                let _rhs = 1;
                let _lhs
                    = &mut (*self).data_[
                               *{
                                    let _old = p;
                                    p = p.offset(1 as (isize));
                                    _old
                                } as (usize)
                           ];
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn InitialEntropyCodesLiteral(
    mut data : *const u8,
    mut length : usize,
    mut stride : usize,
    mut num_histograms : usize,
    mut histograms : *mut HistogramLiteral
) {
    let mut seed : u32 = 7i32 as (u32);
    let mut block_length : usize = length.wrapping_div(num_histograms);
    let mut i : usize;
    ClearHistogramsLiteral(histograms,num_histograms);
    i = 0i32 as (usize);
    'loop1: loop {
        if i < num_histograms {
            let mut pos
                : usize
                = length.wrapping_mul(i).wrapping_div(num_histograms);
            if i != 0i32 as (usize) {
                pos = pos.wrapping_add(
                          (MyRand(&mut seed as (*mut u32)) as (usize)).wrapping_rem(
                              block_length
                          )
                      );
            }
            if pos.wrapping_add(stride) >= length {
                pos = length.wrapping_sub(stride).wrapping_sub(1i32 as (usize));
            }
            HistogramAddVectorLiteral(
                &mut *histograms.offset(i as (isize)) as (*mut HistogramLiteral),
                data.offset(pos as (isize)),
                stride
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn RandomSampleLiteral(
    mut seed : *mut u32,
    mut data : *const u8,
    mut length : usize,
    mut stride : usize,
    mut sample : *mut HistogramLiteral
) {
    let mut pos : usize = 0i32 as (usize);
    if stride >= length {
        pos = 0i32 as (usize);
        stride = length;
    } else {
        pos = (MyRand(seed) as (usize)).wrapping_rem(
                  length.wrapping_sub(stride).wrapping_add(1i32 as (usize))
              );
    }
    HistogramAddVectorLiteral(
        sample,
        data.offset(pos as (isize)),
        stride
    );
}

unsafe extern fn HistogramAddHistogramLiteral(
    mut self : *mut HistogramLiteral, mut v : *const HistogramLiteral
) {
    let mut i : usize;
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               (*v).total_count_
                           );
    i = 0i32 as (usize);
    'loop1: loop {
        if i < 256i32 as (usize) {
            {
                let _rhs = (*v).data_[i];
                let _lhs = &mut (*self).data_[i];
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn RefineEntropyCodesLiteral(
    mut data : *const u8,
    mut length : usize,
    mut stride : usize,
    mut num_histograms : usize,
    mut histograms : *mut HistogramLiteral
) {
    let mut iters
        : usize
        = kIterMulForRefining.wrapping_mul(length).wrapping_div(
              stride
          ).wrapping_add(
              kMinItersForRefining
          );
    let mut seed : u32 = 7i32 as (u32);
    let mut iter : usize;
    iters = iters.wrapping_add(num_histograms).wrapping_sub(
                1i32 as (usize)
            ).wrapping_div(
                num_histograms
            ).wrapping_mul(
                num_histograms
            );
    iter = 0i32 as (usize);
    'loop1: loop {
        if iter < iters {
            let mut sample : HistogramLiteral;
            HistogramClearLiteral(&mut sample as (*mut HistogramLiteral));
            RandomSampleLiteral(
                &mut seed as (*mut u32),
                data,
                length,
                stride,
                &mut sample as (*mut HistogramLiteral)
            );
            HistogramAddHistogramLiteral(
                &mut *histograms.offset(
                          iter.wrapping_rem(num_histograms) as (isize)
                      ) as (*mut HistogramLiteral),
                &mut sample as (*mut HistogramLiteral) as (*const HistogramLiteral)
            );
            iter = iter.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn FastLog2(mut v : usize) -> f64 {
    if v < ::std::mem::size_of::<[f32; 256]>().wrapping_div(
               ::std::mem::size_of::<f32>()
           ) {
        kLog2Table[v] as (f64)
    } else {
        log2(v as (f64))
    }
}

unsafe extern fn BitCost(mut count : usize) -> f64 {
    if count == 0i32 as (usize) { -2.0f64 } else { FastLog2(count) }
}

unsafe extern fn FindBlocksLiteral(
    mut data : *const u8,
    length : usize,
    block_switch_bitcost : f64,
    num_histograms : usize,
    mut histograms : *const HistogramLiteral,
    mut insert_cost : *mut f64,
    mut cost : *mut f64,
    mut switch_signal : *mut u8,
    mut block_id : *mut u8
) -> usize {
    let data_size : usize = HistogramDataSizeLiteral();
    let bitmaplen
        : usize
        = num_histograms.wrapping_add(7i32 as (usize)) >> 3i32;
    let mut num_blocks : usize = 1i32 as (usize);
    let mut i : usize;
    let mut j : usize;
    if num_histograms <= 256i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            (*b"num_histograms <= 256\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"FindBlocksLiteral\0").as_ptr()
        );
    }
    if num_histograms <= 1i32 as (usize) {
        i = 0i32 as (usize);
        'loop34: loop {
            if i < length {
                *block_id.offset(i as (isize)) = 0i32 as (u8);
                i = i.wrapping_add(1 as (usize));
                continue 'loop34;
            } else {
                break 'loop34;
            }
        }
        1i32 as (usize)
    } else {
        memset(
            insert_cost as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<f64>().wrapping_mul(data_size).wrapping_mul(
                num_histograms
            )
        );
        i = 0i32 as (usize);
        'loop2: loop {
            if i < num_histograms {
                *insert_cost.offset(i as (isize)) = FastLog2(
                                                        (*histograms.offset(
                                                              i as (isize)
                                                          )).total_count_ as (u32) as (usize)
                                                    );
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        i = data_size;
        'loop4: loop {
            if i != 0i32 as (usize) {
                i = i.wrapping_sub(1 as (usize));
                j = 0i32 as (usize);
                'loop28: loop {
                    if j < num_histograms {
                        *insert_cost.offset(
                             i.wrapping_mul(num_histograms).wrapping_add(j) as (isize)
                         ) = *insert_cost.offset(j as (isize)) - BitCost(
                                                                     (*histograms.offset(
                                                                           j as (isize)
                                                                       )).data_[
                                                                         i
                                                                     ] as (usize)
                                                                 );
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop28;
                    } else {
                        continue 'loop4;
                    }
                }
            } else {
                break 'loop4;
            }
        }
        memset(
            cost as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<f64>().wrapping_mul(num_histograms)
        );
        memset(
            switch_signal as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<u8>().wrapping_mul(length).wrapping_mul(
                bitmaplen
            )
        );
        i = 0i32 as (usize);
        'loop6: loop {
            if i < length {
                let byte_ix : usize = i;
                let mut ix : usize = byte_ix.wrapping_mul(bitmaplen);
                let mut insert_cost_ix
                    : usize
                    = (*data.offset(byte_ix as (isize)) as (usize)).wrapping_mul(
                          num_histograms
                      );
                let mut min_cost : f64 = 1e99f64;
                let mut block_switch_cost : f64 = block_switch_bitcost;
                let mut k : usize;
                k = 0i32 as (usize);
                'loop15: loop {
                    if k < num_histograms {
                        {
                            let _rhs
                                = *insert_cost.offset(insert_cost_ix.wrapping_add(k) as (isize));
                            let _lhs = &mut *cost.offset(k as (isize));
                            *_lhs = *_lhs + _rhs;
                        }
                        if *cost.offset(k as (isize)) < min_cost {
                            min_cost = *cost.offset(k as (isize));
                            *block_id.offset(byte_ix as (isize)) = k as (u8);
                        }
                        k = k.wrapping_add(1 as (usize));
                        continue 'loop15;
                    } else {
                        break 'loop15;
                    }
                }
                if byte_ix < 2000i32 as (usize) {
                    block_switch_cost = block_switch_cost * (0.77f64 + 0.07f64 * byte_ix as (f64) / 2000i32 as (f64));
                }
                k = 0i32 as (usize);
                'loop19: loop {
                    if k < num_histograms {
                        {
                            let _rhs = min_cost;
                            let _lhs = &mut *cost.offset(k as (isize));
                            *_lhs = *_lhs - _rhs;
                        }
                        if *cost.offset(k as (isize)) >= block_switch_cost {
                            let mask : u8 = (1u32 << (k & 7i32 as (usize))) as (u8);
                            *cost.offset(k as (isize)) = block_switch_cost;
                            if k >> 3i32 < bitmaplen {
                                0i32;
                            } else {
                                __assert_fail(
                                    (*b"(k >> 3) < bitmaplen\0").as_ptr(),
                                    file!().as_ptr(),
                                    line!(),
                                    (*b"FindBlocksLiteral\0").as_ptr()
                                );
                            }
                            {
                                let _rhs = mask;
                                let _lhs
                                    = &mut *switch_signal.offset(
                                                ix.wrapping_add(k >> 3i32) as (isize)
                                            );
                                *_lhs = (*_lhs as (i32) | _rhs as (i32)) as (u8);
                            }
                        }
                        k = k.wrapping_add(1 as (usize));
                        continue 'loop19;
                    } else {
                        break 'loop19;
                    }
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop6;
            } else {
                break 'loop6;
            }
        }
        let mut byte_ix : usize = length.wrapping_sub(1i32 as (usize));
        let mut ix : usize = byte_ix.wrapping_mul(bitmaplen);
        let mut cur_id : u8 = *block_id.offset(byte_ix as (isize));
        'loop8: loop {
            if byte_ix > 0i32 as (usize) {
                let mask : u8 = (1u32 << (cur_id as (i32) & 7i32)) as (u8);
                if cur_id as (usize) >> 3i32 < bitmaplen {
                    0i32;
                } else {
                    __assert_fail(
                        (*b"((size_t)cur_id >> 3) < bitmaplen\0").as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        (*b"FindBlocksLiteral\0").as_ptr()
                    );
                }
                byte_ix = byte_ix.wrapping_sub(1 as (usize));
                ix = ix.wrapping_sub(bitmaplen);
                if *switch_signal.offset(
                        ix.wrapping_add((cur_id as (i32) >> 3i32) as (usize)) as (isize)
                    ) as (i32) & mask as (i32) != 0 {
                    if cur_id as (i32) != *block_id.offset(
                                               byte_ix as (isize)
                                           ) as (i32) {
                        cur_id = *block_id.offset(byte_ix as (isize));
                        num_blocks = num_blocks.wrapping_add(1 as (usize));
                    }
                }
                *block_id.offset(byte_ix as (isize)) = cur_id;
                continue 'loop8;
            } else {
                break 'loop8;
            }
        }
        num_blocks
    }
}

unsafe extern fn RemapBlockIdsLiteral(
    mut block_ids : *mut u8,
    length : usize,
    mut new_id : *mut u16,
    num_histograms : usize
) -> usize {
    static kInvalidId : u16 = 256i32 as (u16);
    let mut next_id : u16 = 0i32 as (u16);
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < num_histograms {
            *new_id.offset(i as (isize)) = kInvalidId;
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    i = 0i32 as (usize);
    'loop3: loop {
        if i < length {
            if *block_ids.offset(i as (isize)) as (usize) < num_histograms {
                0i32;
            } else {
                __assert_fail(
                    (*b"block_ids[i] < num_histograms\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"RemapBlockIdsLiteral\0").as_ptr()
                );
            }
            if *new_id.offset(
                    *block_ids.offset(i as (isize)) as (isize)
                ) as (i32) == kInvalidId as (i32) {
                *new_id.offset(*block_ids.offset(i as (isize)) as (isize)) = {
                                                                                 let _old = next_id;
                                                                                 next_id = (next_id as (i32) + 1) as (u16);
                                                                                 _old
                                                                             };
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    i = 0i32 as (usize);
    'loop5: loop {
        if i < length {
            *block_ids.offset(i as (isize)) = *new_id.offset(
                                                   *block_ids.offset(i as (isize)) as (isize)
                                               ) as (u8);
            if *block_ids.offset(i as (isize)) as (usize) < num_histograms {
                0i32;
            } else {
                __assert_fail(
                    (*b"block_ids[i] < num_histograms\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"RemapBlockIdsLiteral\0").as_ptr()
                );
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop5;
        } else {
            break 'loop5;
        }
    }
    if next_id as (usize) <= num_histograms {
        0i32;
    } else {
        __assert_fail(
            (*b"next_id <= num_histograms\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"RemapBlockIdsLiteral\0").as_ptr()
        );
    }
    next_id as (usize)
}

unsafe extern fn HistogramAddLiteral(
    mut self : *mut HistogramLiteral, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut (*self).data_[val];
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn BuildBlockHistogramsLiteral(
    mut data : *const u8,
    length : usize,
    mut block_ids : *const u8,
    num_histograms : usize,
    mut histograms : *mut HistogramLiteral
) {
    let mut i : usize;
    ClearHistogramsLiteral(histograms,num_histograms);
    i = 0i32 as (usize);
    'loop1: loop {
        if i < length {
            HistogramAddLiteral(
                &mut *histograms.offset(
                          *block_ids.offset(i as (isize)) as (isize)
                      ) as (*mut HistogramLiteral),
                *data.offset(i as (isize)) as (usize)
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramPair {
    pub idx1 : u32,
    pub idx2 : u32,
    pub cost_combo : f64,
    pub cost_diff : f64,
}

unsafe extern fn brotli_max_uint8_t(mut a : u8, mut b : u8) -> u8 {
    (if a as (i32) > b as (i32) {
         a as (i32)
     } else {
         b as (i32)
     }) as (u8)
}

unsafe extern fn ClusterBlocksLiteral(
    mut m : *mut MemoryManager,
    mut data : *const u8,
    length : usize,
    num_blocks : usize,
    mut block_ids : *mut u8,
    mut split : *mut BlockSplit
) {
    let mut histogram_symbols
        : *mut u32
        = if num_blocks != 0 {
              BrotliAllocate(
                  m,
                  num_blocks.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut block_lengths
        : *mut u32
        = if num_blocks != 0 {
              BrotliAllocate(
                  m,
                  num_blocks.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let expected_num_clusters
        : usize
        = (16i32 as (usize)).wrapping_mul(
              num_blocks.wrapping_add(64i32 as (usize)).wrapping_sub(
                  1i32 as (usize)
              )
          ).wrapping_div(
              64i32 as (usize)
          );
    let mut all_histograms_size : usize = 0i32 as (usize);
    let mut all_histograms_capacity : usize = expected_num_clusters;
    let mut all_histograms
        : *mut HistogramLiteral
        = if all_histograms_capacity != 0 {
              BrotliAllocate(
                  m,
                  all_histograms_capacity.wrapping_mul(
                      ::std::mem::size_of::<HistogramLiteral>()
                  )
              ) as (*mut HistogramLiteral)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral)
          };
    let mut cluster_size_size : usize = 0i32 as (usize);
    let mut cluster_size_capacity : usize = expected_num_clusters;
    let mut cluster_size
        : *mut u32
        = if cluster_size_capacity != 0 {
              BrotliAllocate(
                  m,
                  cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut num_clusters : usize = 0i32 as (usize);
    let mut histograms
        : *mut HistogramLiteral
        = if brotli_min_size_t(num_blocks,64i32 as (usize)) != 0 {
              BrotliAllocate(
                  m,
                  brotli_min_size_t(num_blocks,64i32 as (usize)).wrapping_mul(
                      ::std::mem::size_of::<HistogramLiteral>()
                  )
              ) as (*mut HistogramLiteral)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral)
          };
    let mut max_num_pairs : usize = (64i32 * 64i32 / 2i32) as (usize);
    let mut pairs_capacity
        : usize
        = max_num_pairs.wrapping_add(1i32 as (usize));
    let mut pairs
        : *mut HistogramPair
        = if pairs_capacity != 0 {
              BrotliAllocate(
                  m,
                  pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>())
              ) as (*mut HistogramPair)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
          };
    let mut pos : usize = 0i32 as (usize);
    let mut clusters : *mut u32;
    let mut num_final_clusters : usize;
    static kInvalidIndex : u32 = !(0i32 as (u32));
    let mut new_index : *mut u32;
    let mut i : usize;
    let mut sizes
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut new_clusters
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut symbols
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut remap
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    if !(0i32 == 0) {
    } else {
        memset(
            block_lengths as (*mut ::std::os::raw::c_void),
            0i32,
            num_blocks.wrapping_mul(::std::mem::size_of::<u32>())
        );
        let mut block_idx : usize = 0i32 as (usize);
        i = 0i32 as (usize);
        'loop2: loop {
            if i < length {
                if block_idx < num_blocks {
                    0i32;
                } else {
                    __assert_fail(
                        (*b"block_idx < num_blocks\0").as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        (*b"ClusterBlocksLiteral\0").as_ptr()
                    );
                }
                {
                    let _rhs = 1;
                    let _lhs = &mut *block_lengths.offset(block_idx as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                }
                if i.wrapping_add(1i32 as (usize)) == length || *block_ids.offset(
                                                                     i as (isize)
                                                                 ) as (i32) != *block_ids.offset(
                                                                                    i.wrapping_add(
                                                                                        1i32 as (usize)
                                                                                    ) as (isize)
                                                                                ) as (i32) {
                    block_idx = block_idx.wrapping_add(1 as (usize));
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        if block_idx == num_blocks {
            0i32;
        } else {
            __assert_fail(
                (*b"block_idx == num_blocks\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"ClusterBlocksLiteral\0").as_ptr()
            );
        }
        i = 0i32 as (usize);
        'loop4: loop {
            if i < num_blocks {
                let num_to_combine
                    : usize
                    = brotli_min_size_t(num_blocks.wrapping_sub(i),64i32 as (usize));
                let mut num_new_clusters : usize;
                let mut j : usize;
                j = 0i32 as (usize);
                'loop57: loop {
                    if j < num_to_combine {
                        let mut k : usize;
                        HistogramClearLiteral(
                            &mut *histograms.offset(j as (isize)) as (*mut HistogramLiteral)
                        );
                        k = 0i32 as (usize);
                        'loop85: loop {
                            if k < *block_lengths.offset(
                                        i.wrapping_add(j) as (isize)
                                    ) as (usize) {
                                HistogramAddLiteral(
                                    &mut *histograms.offset(
                                              j as (isize)
                                          ) as (*mut HistogramLiteral),
                                    *data.offset(
                                         {
                                             let _old = pos;
                                             pos = pos.wrapping_add(1 as (usize));
                                             _old
                                         } as (isize)
                                     ) as (usize)
                                );
                                k = k.wrapping_add(1 as (usize));
                                continue 'loop85;
                            } else {
                                break 'loop85;
                            }
                        }
                        (*histograms.offset(
                              j as (isize)
                          )).bit_cost_ = BrotliPopulationCostLiteral(
                                             &mut *histograms.offset(
                                                       j as (isize)
                                                   ) as (*mut HistogramLiteral) as (*const HistogramLiteral)
                                         );
                        new_clusters[j] = j as (u32);
                        symbols[j] = j as (u32);
                        sizes[j] = 1i32 as (u32);
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop57;
                    } else {
                        break 'loop57;
                    }
                }
                num_new_clusters = BrotliHistogramCombineLiteral(
                                       histograms,
                                       sizes.as_mut_ptr(),
                                       symbols.as_mut_ptr(),
                                       new_clusters.as_mut_ptr(),
                                       pairs,
                                       num_to_combine,
                                       num_to_combine,
                                       64i32 as (usize),
                                       max_num_pairs
                                   );
                if all_histograms_capacity < all_histograms_size.wrapping_add(
                                                 num_new_clusters
                                             ) {
                    let mut _new_size
                        : usize
                        = if all_histograms_capacity == 0i32 as (usize) {
                              all_histograms_size.wrapping_add(num_new_clusters)
                          } else {
                              all_histograms_capacity
                          };
                    let mut new_array : *mut HistogramLiteral;
                    'loop60: loop {
                        if _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
                            _new_size = _new_size.wrapping_mul(2i32 as (usize));
                            continue 'loop60;
                        } else {
                            break 'loop60;
                        }
                    }
                    new_array = if _new_size != 0 {
                                    BrotliAllocate(
                                        m,
                                        _new_size.wrapping_mul(
                                            ::std::mem::size_of::<HistogramLiteral>()
                                        )
                                    ) as (*mut HistogramLiteral)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral)
                                };
                    if !!(0i32 == 0) && (all_histograms_capacity != 0i32 as (usize)) {
                        memcpy(
                            new_array as (*mut ::std::os::raw::c_void),
                            all_histograms as (*const ::std::os::raw::c_void),
                            all_histograms_capacity.wrapping_mul(
                                ::std::mem::size_of::<HistogramLiteral>()
                            )
                        );
                    }
                    BrotliFree(m,all_histograms as (*mut ::std::os::raw::c_void));
                    all_histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral);
                    all_histograms = new_array;
                    all_histograms_capacity = _new_size;
                }
                if cluster_size_capacity < cluster_size_size.wrapping_add(
                                               num_new_clusters
                                           ) {
                    let mut _new_size
                        : usize
                        = if cluster_size_capacity == 0i32 as (usize) {
                              cluster_size_size.wrapping_add(num_new_clusters)
                          } else {
                              cluster_size_capacity
                          };
                    let mut new_array : *mut u32;
                    'loop66: loop {
                        if _new_size < cluster_size_size.wrapping_add(num_new_clusters) {
                            _new_size = _new_size.wrapping_mul(2i32 as (usize));
                            continue 'loop66;
                        } else {
                            break 'loop66;
                        }
                    }
                    new_array = if _new_size != 0 {
                                    BrotliAllocate(
                                        m,
                                        _new_size.wrapping_mul(::std::mem::size_of::<u32>())
                                    ) as (*mut u32)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                };
                    if !!(0i32 == 0) && (cluster_size_capacity != 0i32 as (usize)) {
                        memcpy(
                            new_array as (*mut ::std::os::raw::c_void),
                            cluster_size as (*const ::std::os::raw::c_void),
                            cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>())
                        );
                    }
                    BrotliFree(m,cluster_size as (*mut ::std::os::raw::c_void));
                    cluster_size = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                    cluster_size = new_array;
                    cluster_size_capacity = _new_size;
                }
                if !(0i32 == 0) {
                    break 'loop4;
                } else {
                    j = 0i32 as (usize);
                    'loop72: loop {
                        if j < num_new_clusters {
                            *all_histograms.offset(
                                 {
                                     let _old = all_histograms_size;
                                     all_histograms_size = all_histograms_size.wrapping_add(
                                                               1 as (usize)
                                                           );
                                     _old
                                 } as (isize)
                             ) = *histograms.offset(new_clusters[j] as (isize));
                            *cluster_size.offset(
                                 {
                                     let _old = cluster_size_size;
                                     cluster_size_size = cluster_size_size.wrapping_add(
                                                             1 as (usize)
                                                         );
                                     _old
                                 } as (isize)
                             ) = sizes[new_clusters[j] as (usize)];
                            remap[new_clusters[j] as (usize)] = j as (u32);
                            j = j.wrapping_add(1 as (usize));
                            continue 'loop72;
                        } else {
                            break 'loop72;
                        }
                    }
                    j = 0i32 as (usize);
                    'loop74: loop {
                        if j < num_to_combine {
                            *histogram_symbols.offset(
                                 i.wrapping_add(j) as (isize)
                             ) = (num_clusters as (u32)).wrapping_add(
                                     remap[symbols[j] as (usize)]
                                 );
                            j = j.wrapping_add(1 as (usize));
                            continue 'loop74;
                        } else {
                            break 'loop74;
                        }
                    }
                    num_clusters = num_clusters.wrapping_add(num_new_clusters);
                    if num_clusters == cluster_size_size {
                        0i32;
                    } else {
                        __assert_fail(
                            (*b"num_clusters == cluster_size_size\0").as_ptr(),
                            file!().as_ptr(),
                            line!(),
                            (*b"ClusterBlocksLiteral\0").as_ptr()
                        );
                    }
                    if num_clusters == all_histograms_size {
                        0i32;
                    } else {
                        __assert_fail(
                            (*b"num_clusters == all_histograms_size\0").as_ptr(),
                            file!().as_ptr(),
                            line!(),
                            (*b"ClusterBlocksLiteral\0").as_ptr()
                        );
                    }
                    i = i.wrapping_add(64i32 as (usize));
                    continue 'loop4;
                }
            } else {
                BrotliFree(m,histograms as (*mut ::std::os::raw::c_void));
                histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral);
                max_num_pairs = brotli_min_size_t(
                                    (64i32 as (usize)).wrapping_mul(num_clusters),
                                    num_clusters.wrapping_div(2i32 as (usize)).wrapping_mul(
                                        num_clusters
                                    )
                                );
                if pairs_capacity < max_num_pairs.wrapping_add(1i32 as (usize)) {
                    BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
                    pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
                    pairs = if max_num_pairs.wrapping_add(1i32 as (usize)) != 0 {
                                BrotliAllocate(
                                    m,
                                    max_num_pairs.wrapping_add(1i32 as (usize)).wrapping_mul(
                                        ::std::mem::size_of::<HistogramPair>()
                                    )
                                ) as (*mut HistogramPair)
                            } else {
                                0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
                            };
                    if !(0i32 == 0) {
                        return;
                    }
                }
                clusters = if num_clusters != 0 {
                               BrotliAllocate(
                                   m,
                                   num_clusters.wrapping_mul(::std::mem::size_of::<u32>())
                               ) as (*mut u32)
                           } else {
                               0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                           };
                if !(0i32 == 0) {
                    return;
                } else {
                    i = 0i32 as (usize);
                    'loop9: loop {
                        if i < num_clusters {
                            *clusters.offset(i as (isize)) = i as (u32);
                            i = i.wrapping_add(1 as (usize));
                            continue 'loop9;
                        } else {
                            break 'loop9;
                        }
                    }
                    num_final_clusters = BrotliHistogramCombineLiteral(
                                             all_histograms,
                                             cluster_size,
                                             histogram_symbols,
                                             clusters,
                                             pairs,
                                             num_clusters,
                                             num_blocks,
                                             256i32 as (usize),
                                             max_num_pairs
                                         );
                    BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
                    pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
                    BrotliFree(m,cluster_size as (*mut ::std::os::raw::c_void));
                    cluster_size = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                    new_index = if num_clusters != 0 {
                                    BrotliAllocate(
                                        m,
                                        num_clusters.wrapping_mul(::std::mem::size_of::<u32>())
                                    ) as (*mut u32)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                };
                    if !(0i32 == 0) {
                        return;
                    } else {
                        i = 0i32 as (usize);
                        'loop12: loop {
                            if i < num_clusters {
                                *new_index.offset(i as (isize)) = kInvalidIndex;
                                i = i.wrapping_add(1 as (usize));
                                continue 'loop12;
                            } else {
                                break 'loop12;
                            }
                        }
                        pos = 0i32 as (usize);
                        let mut next_index : u32 = 0i32 as (u32);
                        i = 0i32 as (usize);
                        'loop14: loop {
                            if i < num_blocks {
                                let mut histo : HistogramLiteral;
                                let mut j : usize;
                                let mut best_out : u32;
                                let mut best_bits : f64;
                                HistogramClearLiteral(&mut histo as (*mut HistogramLiteral));
                                j = 0i32 as (usize);
                                'loop38: loop {
                                    if j < *block_lengths.offset(i as (isize)) as (usize) {
                                        HistogramAddLiteral(
                                            &mut histo as (*mut HistogramLiteral),
                                            *data.offset(
                                                 {
                                                     let _old = pos;
                                                     pos = pos.wrapping_add(1 as (usize));
                                                     _old
                                                 } as (isize)
                                             ) as (usize)
                                        );
                                        j = j.wrapping_add(1 as (usize));
                                        continue 'loop38;
                                    } else {
                                        break 'loop38;
                                    }
                                }
                                best_out = if i == 0i32 as (usize) {
                                               *histogram_symbols.offset(0i32 as (isize))
                                           } else {
                                               *histogram_symbols.offset(
                                                    i.wrapping_sub(1i32 as (usize)) as (isize)
                                                )
                                           };
                                best_bits = BrotliHistogramBitCostDistanceLiteral(
                                                &mut histo as (*mut HistogramLiteral) as (*const HistogramLiteral),
                                                &mut *all_histograms.offset(
                                                          best_out as (isize)
                                                      ) as (*mut HistogramLiteral) as (*const HistogramLiteral)
                                            );
                                j = 0i32 as (usize);
                                'loop40: loop {
                                    if j < num_final_clusters {
                                        let cur_bits
                                            : f64
                                            = BrotliHistogramBitCostDistanceLiteral(
                                                  &mut histo as (*mut HistogramLiteral) as (*const HistogramLiteral),
                                                  &mut *all_histograms.offset(
                                                            *clusters.offset(
                                                                 j as (isize)
                                                             ) as (isize)
                                                        ) as (*mut HistogramLiteral) as (*const HistogramLiteral)
                                              );
                                        if cur_bits < best_bits {
                                            best_bits = cur_bits;
                                            best_out = *clusters.offset(j as (isize));
                                        }
                                        j = j.wrapping_add(1 as (usize));
                                        continue 'loop40;
                                    } else {
                                        break 'loop40;
                                    }
                                }
                                *histogram_symbols.offset(i as (isize)) = best_out;
                                if *new_index.offset(best_out as (isize)) == kInvalidIndex {
                                    *new_index.offset(best_out as (isize)) = {
                                                                                 let _old
                                                                                     = next_index;
                                                                                 next_index = next_index.wrapping_add(
                                                                                                  1 as (u32)
                                                                                              );
                                                                                 _old
                                                                             };
                                }
                                i = i.wrapping_add(1 as (usize));
                                continue 'loop14;
                            } else {
                                break 'loop14;
                            }
                        }
                        BrotliFree(m,clusters as (*mut ::std::os::raw::c_void));
                        clusters = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                        BrotliFree(m,all_histograms as (*mut ::std::os::raw::c_void));
                        all_histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral);
                        if (*split).types_alloc_size < num_blocks {
                            let mut _new_size
                                : usize
                                = if (*split).types_alloc_size == 0i32 as (usize) {
                                      num_blocks
                                  } else {
                                      (*split).types_alloc_size
                                  };
                            let mut new_array : *mut u8;
                            'loop17: loop {
                                if _new_size < num_blocks {
                                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                                    continue 'loop17;
                                } else {
                                    break 'loop17;
                                }
                            }
                            new_array = if _new_size != 0 {
                                            BrotliAllocate(
                                                m,
                                                _new_size.wrapping_mul(::std::mem::size_of::<u8>())
                                            ) as (*mut u8)
                                        } else {
                                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                                        };
                            if !!(0i32 == 0) && ((*split).types_alloc_size != 0i32 as (usize)) {
                                memcpy(
                                    new_array as (*mut ::std::os::raw::c_void),
                                    (*split).types as (*const ::std::os::raw::c_void),
                                    (*split).types_alloc_size.wrapping_mul(
                                        ::std::mem::size_of::<u8>()
                                    )
                                );
                            }
                            BrotliFree(m,(*split).types as (*mut ::std::os::raw::c_void));
                            (*split).types = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                            (*split).types = new_array;
                            (*split).types_alloc_size = _new_size;
                        }
                        if (*split).lengths_alloc_size < num_blocks {
                            let mut _new_size
                                : usize
                                = if (*split).lengths_alloc_size == 0i32 as (usize) {
                                      num_blocks
                                  } else {
                                      (*split).lengths_alloc_size
                                  };
                            let mut new_array : *mut u32;
                            'loop23: loop {
                                if _new_size < num_blocks {
                                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                                    continue 'loop23;
                                } else {
                                    break 'loop23;
                                }
                            }
                            new_array = if _new_size != 0 {
                                            BrotliAllocate(
                                                m,
                                                _new_size.wrapping_mul(::std::mem::size_of::<u32>())
                                            ) as (*mut u32)
                                        } else {
                                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                        };
                            if !!(0i32 == 0) && ((*split).lengths_alloc_size != 0i32 as (usize)) {
                                memcpy(
                                    new_array as (*mut ::std::os::raw::c_void),
                                    (*split).lengths as (*const ::std::os::raw::c_void),
                                    (*split).lengths_alloc_size.wrapping_mul(
                                        ::std::mem::size_of::<u32>()
                                    )
                                );
                            }
                            BrotliFree(m,(*split).lengths as (*mut ::std::os::raw::c_void));
                            (*split).lengths = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                            (*split).lengths = new_array;
                            (*split).lengths_alloc_size = _new_size;
                        }
                        if !(0i32 == 0) {
                            return;
                        } else {
                            let mut cur_length : u32 = 0i32 as (u32);
                            let mut block_idx : usize = 0i32 as (usize);
                            let mut max_type : u8 = 0i32 as (u8);
                            i = 0i32 as (usize);
                            'loop29: loop {
                                if i < num_blocks {
                                    cur_length = cur_length.wrapping_add(
                                                     *block_lengths.offset(i as (isize))
                                                 );
                                    if i.wrapping_add(
                                           1i32 as (usize)
                                       ) == num_blocks || *histogram_symbols.offset(
                                                               i as (isize)
                                                           ) != *histogram_symbols.offset(
                                                                     i.wrapping_add(
                                                                         1i32 as (usize)
                                                                     ) as (isize)
                                                                 ) {
                                        let id
                                            : u8
                                            = *new_index.offset(
                                                   *histogram_symbols.offset(
                                                        i as (isize)
                                                    ) as (isize)
                                               ) as (u8);
                                        *(*split).types.offset(block_idx as (isize)) = id;
                                        *(*split).lengths.offset(block_idx as (isize)) = cur_length;
                                        max_type = brotli_max_uint8_t(max_type,id);
                                        cur_length = 0i32 as (u32);
                                        block_idx = block_idx.wrapping_add(1 as (usize));
                                    }
                                    i = i.wrapping_add(1 as (usize));
                                    continue 'loop29;
                                } else {
                                    break 'loop29;
                                }
                            }
                            (*split).num_blocks = block_idx;
                            (*split).num_types = (max_type as (usize)).wrapping_add(
                                                     1i32 as (usize)
                                                 );
                            BrotliFree(m,new_index as (*mut ::std::os::raw::c_void));
                            new_index = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                            BrotliFree(m,block_lengths as (*mut ::std::os::raw::c_void));
                            block_lengths = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                            BrotliFree(m,histogram_symbols as (*mut ::std::os::raw::c_void));
                            histogram_symbols = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                        }
                    }
                }
            }
        }
    }
}

unsafe extern fn SplitByteVectorLiteral(
    mut m : *mut MemoryManager,
    mut data : *const u8,
    length : usize,
    literals_per_histogram : usize,
    max_histograms : usize,
    sampling_stride_length : usize,
    block_switch_cost : f64,
    mut params : *const BrotliEncoderParams,
    mut split : *mut BlockSplit
) {
    let data_size : usize = HistogramDataSizeLiteral();
    let mut num_histograms
        : usize
        = length.wrapping_div(literals_per_histogram).wrapping_add(
              1i32 as (usize)
          );
    let mut histograms : *mut HistogramLiteral;
    if num_histograms > max_histograms {
        num_histograms = max_histograms;
    }
    if length == 0i32 as (usize) {
        (*split).num_types = 1i32 as (usize);
    } else if length < kMinLengthForBlockSplitting {
        if (*split).types_alloc_size < (*split).num_blocks.wrapping_add(
                                           1i32 as (usize)
                                       ) {
            let mut _new_size
                : usize
                = if (*split).types_alloc_size == 0i32 as (usize) {
                      (*split).num_blocks.wrapping_add(1i32 as (usize))
                  } else {
                      (*split).types_alloc_size
                  };
            let mut new_array : *mut u8;
            'loop17: loop {
                if _new_size < (*split).num_blocks.wrapping_add(1i32 as (usize)) {
                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                    continue 'loop17;
                } else {
                    break 'loop17;
                }
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(::std::mem::size_of::<u8>())
                            ) as (*mut u8)
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                        };
            if !!(0i32 == 0) && ((*split).types_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut ::std::os::raw::c_void),
                    (*split).types as (*const ::std::os::raw::c_void),
                    (*split).types_alloc_size.wrapping_mul(::std::mem::size_of::<u8>())
                );
            }
            BrotliFree(m,(*split).types as (*mut ::std::os::raw::c_void));
            (*split).types = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
            (*split).types = new_array;
            (*split).types_alloc_size = _new_size;
        }
        if (*split).lengths_alloc_size < (*split).num_blocks.wrapping_add(
                                             1i32 as (usize)
                                         ) {
            let mut _new_size
                : usize
                = if (*split).lengths_alloc_size == 0i32 as (usize) {
                      (*split).num_blocks.wrapping_add(1i32 as (usize))
                  } else {
                      (*split).lengths_alloc_size
                  };
            let mut new_array : *mut u32;
            'loop23: loop {
                if _new_size < (*split).num_blocks.wrapping_add(1i32 as (usize)) {
                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                    continue 'loop23;
                } else {
                    break 'loop23;
                }
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(::std::mem::size_of::<u32>())
                            ) as (*mut u32)
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                        };
            if !!(0i32 == 0) && ((*split).lengths_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut ::std::os::raw::c_void),
                    (*split).lengths as (*const ::std::os::raw::c_void),
                    (*split).lengths_alloc_size.wrapping_mul(
                        ::std::mem::size_of::<u32>()
                    )
                );
            }
            BrotliFree(m,(*split).lengths as (*mut ::std::os::raw::c_void));
            (*split).lengths = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            (*split).lengths = new_array;
            (*split).lengths_alloc_size = _new_size;
        }
        if !(0i32 == 0) {
        } else {
            (*split).num_types = 1i32 as (usize);
            *(*split).types.offset(
                 (*split).num_blocks as (isize)
             ) = 0i32 as (u8);
            *(*split).lengths.offset(
                 (*split).num_blocks as (isize)
             ) = length as (u32);
            (*split).num_blocks = (*split).num_blocks.wrapping_add(
                                      1 as (usize)
                                  );
        }
    } else {
        histograms = if num_histograms != 0 {
                         BrotliAllocate(
                             m,
                             num_histograms.wrapping_mul(
                                 ::std::mem::size_of::<HistogramLiteral>()
                             )
                         ) as (*mut HistogramLiteral)
                     } else {
                         0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral)
                     };
        if !(0i32 == 0) {
        } else {
            InitialEntropyCodesLiteral(
                data,
                length,
                sampling_stride_length,
                num_histograms,
                histograms
            );
            RefineEntropyCodesLiteral(
                data,
                length,
                sampling_stride_length,
                num_histograms,
                histograms
            );
            let mut block_ids
                : *mut u8
                = if length != 0 {
                      BrotliAllocate(
                          m,
                          length.wrapping_mul(::std::mem::size_of::<u8>())
                      ) as (*mut u8)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                  };
            let mut num_blocks : usize = 0i32 as (usize);
            let bitmaplen
                : usize
                = num_histograms.wrapping_add(7i32 as (usize)) >> 3i32;
            let mut insert_cost
                : *mut f64
                = if data_size.wrapping_mul(num_histograms) != 0 {
                      BrotliAllocate(
                          m,
                          data_size.wrapping_mul(num_histograms).wrapping_mul(
                              ::std::mem::size_of::<f64>()
                          )
                      ) as (*mut f64)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut f64)
                  };
            let mut cost
                : *mut f64
                = if num_histograms != 0 {
                      BrotliAllocate(
                          m,
                          num_histograms.wrapping_mul(::std::mem::size_of::<f64>())
                      ) as (*mut f64)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut f64)
                  };
            let mut switch_signal
                : *mut u8
                = if length.wrapping_mul(bitmaplen) != 0 {
                      BrotliAllocate(
                          m,
                          length.wrapping_mul(bitmaplen).wrapping_mul(
                              ::std::mem::size_of::<u8>()
                          )
                      ) as (*mut u8)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                  };
            let mut new_id
                : *mut u16
                = if num_histograms != 0 {
                      BrotliAllocate(
                          m,
                          num_histograms.wrapping_mul(::std::mem::size_of::<u16>())
                      ) as (*mut u16)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
                  };
            let iters
                : usize
                = (if (*params).quality < 11i32 {
                       3i32
                   } else {
                       10i32
                   }) as (usize);
            let mut i : usize;
            if !(0i32 == 0) {
            } else {
                i = 0i32 as (usize);
                'loop7: loop {
                    if i < iters {
                        num_blocks = FindBlocksLiteral(
                                         data,
                                         length,
                                         block_switch_cost,
                                         num_histograms,
                                         histograms as (*const HistogramLiteral),
                                         insert_cost,
                                         cost,
                                         switch_signal,
                                         block_ids
                                     );
                        num_histograms = RemapBlockIdsLiteral(
                                             block_ids,
                                             length,
                                             new_id,
                                             num_histograms
                                         );
                        BuildBlockHistogramsLiteral(
                            data,
                            length,
                            block_ids as (*const u8),
                            num_histograms,
                            histograms
                        );
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop7;
                    } else {
                        break 'loop7;
                    }
                }
                BrotliFree(m,insert_cost as (*mut ::std::os::raw::c_void));
                insert_cost = 0i32 as (*mut ::std::os::raw::c_void) as (*mut f64);
                BrotliFree(m,cost as (*mut ::std::os::raw::c_void));
                cost = 0i32 as (*mut ::std::os::raw::c_void) as (*mut f64);
                BrotliFree(m,switch_signal as (*mut ::std::os::raw::c_void));
                switch_signal = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                BrotliFree(m,new_id as (*mut ::std::os::raw::c_void));
                new_id = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16);
                BrotliFree(m,histograms as (*mut ::std::os::raw::c_void));
                histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral);
                ClusterBlocksLiteral(m,data,length,num_blocks,block_ids,split);
                if !(0i32 == 0) {
                } else {
                    BrotliFree(m,block_ids as (*mut ::std::os::raw::c_void));
                    block_ids = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                }
            }
        }
    }
}

unsafe extern fn HistogramDataSizeCommand() -> usize {
    704i32 as (usize)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_ : [u32; 704],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

unsafe extern fn HistogramClearCommand(
    mut self : *mut HistogramCommand
) {
    memset(
        (*self).data_.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 704]>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn ClearHistogramsCommand(
    mut array : *mut HistogramCommand, mut length : usize
) {
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < length {
            HistogramClearCommand(array.offset(i as (isize)));
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn HistogramAddVectorCommand(
    mut self : *mut HistogramCommand, mut p : *const u16, mut n : usize
) {
    (*self).total_count_ = (*self).total_count_.wrapping_add(n);
    n = n.wrapping_add(1i32 as (usize));
    'loop1: loop {
        if {
               n = n.wrapping_sub(1 as (usize));
               n
           } != 0 {
            {
                let _rhs = 1;
                let _lhs
                    = &mut (*self).data_[
                               *{
                                    let _old = p;
                                    p = p.offset(1 as (isize));
                                    _old
                                } as (usize)
                           ];
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn InitialEntropyCodesCommand(
    mut data : *const u16,
    mut length : usize,
    mut stride : usize,
    mut num_histograms : usize,
    mut histograms : *mut HistogramCommand
) {
    let mut seed : u32 = 7i32 as (u32);
    let mut block_length : usize = length.wrapping_div(num_histograms);
    let mut i : usize;
    ClearHistogramsCommand(histograms,num_histograms);
    i = 0i32 as (usize);
    'loop1: loop {
        if i < num_histograms {
            let mut pos
                : usize
                = length.wrapping_mul(i).wrapping_div(num_histograms);
            if i != 0i32 as (usize) {
                pos = pos.wrapping_add(
                          (MyRand(&mut seed as (*mut u32)) as (usize)).wrapping_rem(
                              block_length
                          )
                      );
            }
            if pos.wrapping_add(stride) >= length {
                pos = length.wrapping_sub(stride).wrapping_sub(1i32 as (usize));
            }
            HistogramAddVectorCommand(
                &mut *histograms.offset(i as (isize)) as (*mut HistogramCommand),
                data.offset(pos as (isize)),
                stride
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn RandomSampleCommand(
    mut seed : *mut u32,
    mut data : *const u16,
    mut length : usize,
    mut stride : usize,
    mut sample : *mut HistogramCommand
) {
    let mut pos : usize = 0i32 as (usize);
    if stride >= length {
        pos = 0i32 as (usize);
        stride = length;
    } else {
        pos = (MyRand(seed) as (usize)).wrapping_rem(
                  length.wrapping_sub(stride).wrapping_add(1i32 as (usize))
              );
    }
    HistogramAddVectorCommand(
        sample,
        data.offset(pos as (isize)),
        stride
    );
}

unsafe extern fn HistogramAddHistogramCommand(
    mut self : *mut HistogramCommand, mut v : *const HistogramCommand
) {
    let mut i : usize;
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               (*v).total_count_
                           );
    i = 0i32 as (usize);
    'loop1: loop {
        if i < 704i32 as (usize) {
            {
                let _rhs = (*v).data_[i];
                let _lhs = &mut (*self).data_[i];
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn RefineEntropyCodesCommand(
    mut data : *const u16,
    mut length : usize,
    mut stride : usize,
    mut num_histograms : usize,
    mut histograms : *mut HistogramCommand
) {
    let mut iters
        : usize
        = kIterMulForRefining.wrapping_mul(length).wrapping_div(
              stride
          ).wrapping_add(
              kMinItersForRefining
          );
    let mut seed : u32 = 7i32 as (u32);
    let mut iter : usize;
    iters = iters.wrapping_add(num_histograms).wrapping_sub(
                1i32 as (usize)
            ).wrapping_div(
                num_histograms
            ).wrapping_mul(
                num_histograms
            );
    iter = 0i32 as (usize);
    'loop1: loop {
        if iter < iters {
            let mut sample : HistogramCommand;
            HistogramClearCommand(&mut sample as (*mut HistogramCommand));
            RandomSampleCommand(
                &mut seed as (*mut u32),
                data,
                length,
                stride,
                &mut sample as (*mut HistogramCommand)
            );
            HistogramAddHistogramCommand(
                &mut *histograms.offset(
                          iter.wrapping_rem(num_histograms) as (isize)
                      ) as (*mut HistogramCommand),
                &mut sample as (*mut HistogramCommand) as (*const HistogramCommand)
            );
            iter = iter.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn FindBlocksCommand(
    mut data : *const u16,
    length : usize,
    block_switch_bitcost : f64,
    num_histograms : usize,
    mut histograms : *const HistogramCommand,
    mut insert_cost : *mut f64,
    mut cost : *mut f64,
    mut switch_signal : *mut u8,
    mut block_id : *mut u8
) -> usize {
    let data_size : usize = HistogramDataSizeCommand();
    let bitmaplen
        : usize
        = num_histograms.wrapping_add(7i32 as (usize)) >> 3i32;
    let mut num_blocks : usize = 1i32 as (usize);
    let mut i : usize;
    let mut j : usize;
    if num_histograms <= 256i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            (*b"num_histograms <= 256\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"FindBlocksCommand\0").as_ptr()
        );
    }
    if num_histograms <= 1i32 as (usize) {
        i = 0i32 as (usize);
        'loop34: loop {
            if i < length {
                *block_id.offset(i as (isize)) = 0i32 as (u8);
                i = i.wrapping_add(1 as (usize));
                continue 'loop34;
            } else {
                break 'loop34;
            }
        }
        1i32 as (usize)
    } else {
        memset(
            insert_cost as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<f64>().wrapping_mul(data_size).wrapping_mul(
                num_histograms
            )
        );
        i = 0i32 as (usize);
        'loop2: loop {
            if i < num_histograms {
                *insert_cost.offset(i as (isize)) = FastLog2(
                                                        (*histograms.offset(
                                                              i as (isize)
                                                          )).total_count_ as (u32) as (usize)
                                                    );
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        i = data_size;
        'loop4: loop {
            if i != 0i32 as (usize) {
                i = i.wrapping_sub(1 as (usize));
                j = 0i32 as (usize);
                'loop28: loop {
                    if j < num_histograms {
                        *insert_cost.offset(
                             i.wrapping_mul(num_histograms).wrapping_add(j) as (isize)
                         ) = *insert_cost.offset(j as (isize)) - BitCost(
                                                                     (*histograms.offset(
                                                                           j as (isize)
                                                                       )).data_[
                                                                         i
                                                                     ] as (usize)
                                                                 );
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop28;
                    } else {
                        continue 'loop4;
                    }
                }
            } else {
                break 'loop4;
            }
        }
        memset(
            cost as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<f64>().wrapping_mul(num_histograms)
        );
        memset(
            switch_signal as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<u8>().wrapping_mul(length).wrapping_mul(
                bitmaplen
            )
        );
        i = 0i32 as (usize);
        'loop6: loop {
            if i < length {
                let byte_ix : usize = i;
                let mut ix : usize = byte_ix.wrapping_mul(bitmaplen);
                let mut insert_cost_ix
                    : usize
                    = (*data.offset(byte_ix as (isize)) as (usize)).wrapping_mul(
                          num_histograms
                      );
                let mut min_cost : f64 = 1e99f64;
                let mut block_switch_cost : f64 = block_switch_bitcost;
                let mut k : usize;
                k = 0i32 as (usize);
                'loop15: loop {
                    if k < num_histograms {
                        {
                            let _rhs
                                = *insert_cost.offset(insert_cost_ix.wrapping_add(k) as (isize));
                            let _lhs = &mut *cost.offset(k as (isize));
                            *_lhs = *_lhs + _rhs;
                        }
                        if *cost.offset(k as (isize)) < min_cost {
                            min_cost = *cost.offset(k as (isize));
                            *block_id.offset(byte_ix as (isize)) = k as (u8);
                        }
                        k = k.wrapping_add(1 as (usize));
                        continue 'loop15;
                    } else {
                        break 'loop15;
                    }
                }
                if byte_ix < 2000i32 as (usize) {
                    block_switch_cost = block_switch_cost * (0.77f64 + 0.07f64 * byte_ix as (f64) / 2000i32 as (f64));
                }
                k = 0i32 as (usize);
                'loop19: loop {
                    if k < num_histograms {
                        {
                            let _rhs = min_cost;
                            let _lhs = &mut *cost.offset(k as (isize));
                            *_lhs = *_lhs - _rhs;
                        }
                        if *cost.offset(k as (isize)) >= block_switch_cost {
                            let mask : u8 = (1u32 << (k & 7i32 as (usize))) as (u8);
                            *cost.offset(k as (isize)) = block_switch_cost;
                            if k >> 3i32 < bitmaplen {
                                0i32;
                            } else {
                                __assert_fail(
                                    (*b"(k >> 3) < bitmaplen\0").as_ptr(),
                                    file!().as_ptr(),
                                    line!(),
                                    (*b"FindBlocksCommand\0").as_ptr()
                                );
                            }
                            {
                                let _rhs = mask;
                                let _lhs
                                    = &mut *switch_signal.offset(
                                                ix.wrapping_add(k >> 3i32) as (isize)
                                            );
                                *_lhs = (*_lhs as (i32) | _rhs as (i32)) as (u8);
                            }
                        }
                        k = k.wrapping_add(1 as (usize));
                        continue 'loop19;
                    } else {
                        break 'loop19;
                    }
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop6;
            } else {
                break 'loop6;
            }
        }
        let mut byte_ix : usize = length.wrapping_sub(1i32 as (usize));
        let mut ix : usize = byte_ix.wrapping_mul(bitmaplen);
        let mut cur_id : u8 = *block_id.offset(byte_ix as (isize));
        'loop8: loop {
            if byte_ix > 0i32 as (usize) {
                let mask : u8 = (1u32 << (cur_id as (i32) & 7i32)) as (u8);
                if cur_id as (usize) >> 3i32 < bitmaplen {
                    0i32;
                } else {
                    __assert_fail(
                        (*b"((size_t)cur_id >> 3) < bitmaplen\0").as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        (*b"FindBlocksCommand\0").as_ptr()
                    );
                }
                byte_ix = byte_ix.wrapping_sub(1 as (usize));
                ix = ix.wrapping_sub(bitmaplen);
                if *switch_signal.offset(
                        ix.wrapping_add((cur_id as (i32) >> 3i32) as (usize)) as (isize)
                    ) as (i32) & mask as (i32) != 0 {
                    if cur_id as (i32) != *block_id.offset(
                                               byte_ix as (isize)
                                           ) as (i32) {
                        cur_id = *block_id.offset(byte_ix as (isize));
                        num_blocks = num_blocks.wrapping_add(1 as (usize));
                    }
                }
                *block_id.offset(byte_ix as (isize)) = cur_id;
                continue 'loop8;
            } else {
                break 'loop8;
            }
        }
        num_blocks
    }
}

unsafe extern fn RemapBlockIdsCommand(
    mut block_ids : *mut u8,
    length : usize,
    mut new_id : *mut u16,
    num_histograms : usize
) -> usize {
    static kInvalidId : u16 = 256i32 as (u16);
    let mut next_id : u16 = 0i32 as (u16);
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < num_histograms {
            *new_id.offset(i as (isize)) = kInvalidId;
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    i = 0i32 as (usize);
    'loop3: loop {
        if i < length {
            if *block_ids.offset(i as (isize)) as (usize) < num_histograms {
                0i32;
            } else {
                __assert_fail(
                    (*b"block_ids[i] < num_histograms\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"RemapBlockIdsCommand\0").as_ptr()
                );
            }
            if *new_id.offset(
                    *block_ids.offset(i as (isize)) as (isize)
                ) as (i32) == kInvalidId as (i32) {
                *new_id.offset(*block_ids.offset(i as (isize)) as (isize)) = {
                                                                                 let _old = next_id;
                                                                                 next_id = (next_id as (i32) + 1) as (u16);
                                                                                 _old
                                                                             };
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    i = 0i32 as (usize);
    'loop5: loop {
        if i < length {
            *block_ids.offset(i as (isize)) = *new_id.offset(
                                                   *block_ids.offset(i as (isize)) as (isize)
                                               ) as (u8);
            if *block_ids.offset(i as (isize)) as (usize) < num_histograms {
                0i32;
            } else {
                __assert_fail(
                    (*b"block_ids[i] < num_histograms\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"RemapBlockIdsCommand\0").as_ptr()
                );
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop5;
        } else {
            break 'loop5;
        }
    }
    if next_id as (usize) <= num_histograms {
        0i32;
    } else {
        __assert_fail(
            (*b"next_id <= num_histograms\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"RemapBlockIdsCommand\0").as_ptr()
        );
    }
    next_id as (usize)
}

unsafe extern fn HistogramAddCommand(
    mut self : *mut HistogramCommand, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut (*self).data_[val];
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn BuildBlockHistogramsCommand(
    mut data : *const u16,
    length : usize,
    mut block_ids : *const u8,
    num_histograms : usize,
    mut histograms : *mut HistogramCommand
) {
    let mut i : usize;
    ClearHistogramsCommand(histograms,num_histograms);
    i = 0i32 as (usize);
    'loop1: loop {
        if i < length {
            HistogramAddCommand(
                &mut *histograms.offset(
                          *block_ids.offset(i as (isize)) as (isize)
                      ) as (*mut HistogramCommand),
                *data.offset(i as (isize)) as (usize)
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn ClusterBlocksCommand(
    mut m : *mut MemoryManager,
    mut data : *const u16,
    length : usize,
    num_blocks : usize,
    mut block_ids : *mut u8,
    mut split : *mut BlockSplit
) {
    let mut histogram_symbols
        : *mut u32
        = if num_blocks != 0 {
              BrotliAllocate(
                  m,
                  num_blocks.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut block_lengths
        : *mut u32
        = if num_blocks != 0 {
              BrotliAllocate(
                  m,
                  num_blocks.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let expected_num_clusters
        : usize
        = (16i32 as (usize)).wrapping_mul(
              num_blocks.wrapping_add(64i32 as (usize)).wrapping_sub(
                  1i32 as (usize)
              )
          ).wrapping_div(
              64i32 as (usize)
          );
    let mut all_histograms_size : usize = 0i32 as (usize);
    let mut all_histograms_capacity : usize = expected_num_clusters;
    let mut all_histograms
        : *mut HistogramCommand
        = if all_histograms_capacity != 0 {
              BrotliAllocate(
                  m,
                  all_histograms_capacity.wrapping_mul(
                      ::std::mem::size_of::<HistogramCommand>()
                  )
              ) as (*mut HistogramCommand)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand)
          };
    let mut cluster_size_size : usize = 0i32 as (usize);
    let mut cluster_size_capacity : usize = expected_num_clusters;
    let mut cluster_size
        : *mut u32
        = if cluster_size_capacity != 0 {
              BrotliAllocate(
                  m,
                  cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut num_clusters : usize = 0i32 as (usize);
    let mut histograms
        : *mut HistogramCommand
        = if brotli_min_size_t(num_blocks,64i32 as (usize)) != 0 {
              BrotliAllocate(
                  m,
                  brotli_min_size_t(num_blocks,64i32 as (usize)).wrapping_mul(
                      ::std::mem::size_of::<HistogramCommand>()
                  )
              ) as (*mut HistogramCommand)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand)
          };
    let mut max_num_pairs : usize = (64i32 * 64i32 / 2i32) as (usize);
    let mut pairs_capacity
        : usize
        = max_num_pairs.wrapping_add(1i32 as (usize));
    let mut pairs
        : *mut HistogramPair
        = if pairs_capacity != 0 {
              BrotliAllocate(
                  m,
                  pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>())
              ) as (*mut HistogramPair)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
          };
    let mut pos : usize = 0i32 as (usize);
    let mut clusters : *mut u32;
    let mut num_final_clusters : usize;
    static kInvalidIndex : u32 = !(0i32 as (u32));
    let mut new_index : *mut u32;
    let mut i : usize;
    let mut sizes
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut new_clusters
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut symbols
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut remap
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    if !(0i32 == 0) {
    } else {
        memset(
            block_lengths as (*mut ::std::os::raw::c_void),
            0i32,
            num_blocks.wrapping_mul(::std::mem::size_of::<u32>())
        );
        let mut block_idx : usize = 0i32 as (usize);
        i = 0i32 as (usize);
        'loop2: loop {
            if i < length {
                if block_idx < num_blocks {
                    0i32;
                } else {
                    __assert_fail(
                        (*b"block_idx < num_blocks\0").as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        (*b"ClusterBlocksCommand\0").as_ptr()
                    );
                }
                {
                    let _rhs = 1;
                    let _lhs = &mut *block_lengths.offset(block_idx as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                }
                if i.wrapping_add(1i32 as (usize)) == length || *block_ids.offset(
                                                                     i as (isize)
                                                                 ) as (i32) != *block_ids.offset(
                                                                                    i.wrapping_add(
                                                                                        1i32 as (usize)
                                                                                    ) as (isize)
                                                                                ) as (i32) {
                    block_idx = block_idx.wrapping_add(1 as (usize));
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        if block_idx == num_blocks {
            0i32;
        } else {
            __assert_fail(
                (*b"block_idx == num_blocks\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"ClusterBlocksCommand\0").as_ptr()
            );
        }
        i = 0i32 as (usize);
        'loop4: loop {
            if i < num_blocks {
                let num_to_combine
                    : usize
                    = brotli_min_size_t(num_blocks.wrapping_sub(i),64i32 as (usize));
                let mut num_new_clusters : usize;
                let mut j : usize;
                j = 0i32 as (usize);
                'loop57: loop {
                    if j < num_to_combine {
                        let mut k : usize;
                        HistogramClearCommand(
                            &mut *histograms.offset(j as (isize)) as (*mut HistogramCommand)
                        );
                        k = 0i32 as (usize);
                        'loop85: loop {
                            if k < *block_lengths.offset(
                                        i.wrapping_add(j) as (isize)
                                    ) as (usize) {
                                HistogramAddCommand(
                                    &mut *histograms.offset(
                                              j as (isize)
                                          ) as (*mut HistogramCommand),
                                    *data.offset(
                                         {
                                             let _old = pos;
                                             pos = pos.wrapping_add(1 as (usize));
                                             _old
                                         } as (isize)
                                     ) as (usize)
                                );
                                k = k.wrapping_add(1 as (usize));
                                continue 'loop85;
                            } else {
                                break 'loop85;
                            }
                        }
                        (*histograms.offset(
                              j as (isize)
                          )).bit_cost_ = BrotliPopulationCostCommand(
                                             &mut *histograms.offset(
                                                       j as (isize)
                                                   ) as (*mut HistogramCommand) as (*const HistogramCommand)
                                         );
                        new_clusters[j] = j as (u32);
                        symbols[j] = j as (u32);
                        sizes[j] = 1i32 as (u32);
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop57;
                    } else {
                        break 'loop57;
                    }
                }
                num_new_clusters = BrotliHistogramCombineCommand(
                                       histograms,
                                       sizes.as_mut_ptr(),
                                       symbols.as_mut_ptr(),
                                       new_clusters.as_mut_ptr(),
                                       pairs,
                                       num_to_combine,
                                       num_to_combine,
                                       64i32 as (usize),
                                       max_num_pairs
                                   );
                if all_histograms_capacity < all_histograms_size.wrapping_add(
                                                 num_new_clusters
                                             ) {
                    let mut _new_size
                        : usize
                        = if all_histograms_capacity == 0i32 as (usize) {
                              all_histograms_size.wrapping_add(num_new_clusters)
                          } else {
                              all_histograms_capacity
                          };
                    let mut new_array : *mut HistogramCommand;
                    'loop60: loop {
                        if _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
                            _new_size = _new_size.wrapping_mul(2i32 as (usize));
                            continue 'loop60;
                        } else {
                            break 'loop60;
                        }
                    }
                    new_array = if _new_size != 0 {
                                    BrotliAllocate(
                                        m,
                                        _new_size.wrapping_mul(
                                            ::std::mem::size_of::<HistogramCommand>()
                                        )
                                    ) as (*mut HistogramCommand)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand)
                                };
                    if !!(0i32 == 0) && (all_histograms_capacity != 0i32 as (usize)) {
                        memcpy(
                            new_array as (*mut ::std::os::raw::c_void),
                            all_histograms as (*const ::std::os::raw::c_void),
                            all_histograms_capacity.wrapping_mul(
                                ::std::mem::size_of::<HistogramCommand>()
                            )
                        );
                    }
                    BrotliFree(m,all_histograms as (*mut ::std::os::raw::c_void));
                    all_histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand);
                    all_histograms = new_array;
                    all_histograms_capacity = _new_size;
                }
                if cluster_size_capacity < cluster_size_size.wrapping_add(
                                               num_new_clusters
                                           ) {
                    let mut _new_size
                        : usize
                        = if cluster_size_capacity == 0i32 as (usize) {
                              cluster_size_size.wrapping_add(num_new_clusters)
                          } else {
                              cluster_size_capacity
                          };
                    let mut new_array : *mut u32;
                    'loop66: loop {
                        if _new_size < cluster_size_size.wrapping_add(num_new_clusters) {
                            _new_size = _new_size.wrapping_mul(2i32 as (usize));
                            continue 'loop66;
                        } else {
                            break 'loop66;
                        }
                    }
                    new_array = if _new_size != 0 {
                                    BrotliAllocate(
                                        m,
                                        _new_size.wrapping_mul(::std::mem::size_of::<u32>())
                                    ) as (*mut u32)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                };
                    if !!(0i32 == 0) && (cluster_size_capacity != 0i32 as (usize)) {
                        memcpy(
                            new_array as (*mut ::std::os::raw::c_void),
                            cluster_size as (*const ::std::os::raw::c_void),
                            cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>())
                        );
                    }
                    BrotliFree(m,cluster_size as (*mut ::std::os::raw::c_void));
                    cluster_size = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                    cluster_size = new_array;
                    cluster_size_capacity = _new_size;
                }
                if !(0i32 == 0) {
                    break 'loop4;
                } else {
                    j = 0i32 as (usize);
                    'loop72: loop {
                        if j < num_new_clusters {
                            *all_histograms.offset(
                                 {
                                     let _old = all_histograms_size;
                                     all_histograms_size = all_histograms_size.wrapping_add(
                                                               1 as (usize)
                                                           );
                                     _old
                                 } as (isize)
                             ) = *histograms.offset(new_clusters[j] as (isize));
                            *cluster_size.offset(
                                 {
                                     let _old = cluster_size_size;
                                     cluster_size_size = cluster_size_size.wrapping_add(
                                                             1 as (usize)
                                                         );
                                     _old
                                 } as (isize)
                             ) = sizes[new_clusters[j] as (usize)];
                            remap[new_clusters[j] as (usize)] = j as (u32);
                            j = j.wrapping_add(1 as (usize));
                            continue 'loop72;
                        } else {
                            break 'loop72;
                        }
                    }
                    j = 0i32 as (usize);
                    'loop74: loop {
                        if j < num_to_combine {
                            *histogram_symbols.offset(
                                 i.wrapping_add(j) as (isize)
                             ) = (num_clusters as (u32)).wrapping_add(
                                     remap[symbols[j] as (usize)]
                                 );
                            j = j.wrapping_add(1 as (usize));
                            continue 'loop74;
                        } else {
                            break 'loop74;
                        }
                    }
                    num_clusters = num_clusters.wrapping_add(num_new_clusters);
                    if num_clusters == cluster_size_size {
                        0i32;
                    } else {
                        __assert_fail(
                            (*b"num_clusters == cluster_size_size\0").as_ptr(),
                            file!().as_ptr(),
                            line!(),
                            (*b"ClusterBlocksCommand\0").as_ptr()
                        );
                    }
                    if num_clusters == all_histograms_size {
                        0i32;
                    } else {
                        __assert_fail(
                            (*b"num_clusters == all_histograms_size\0").as_ptr(),
                            file!().as_ptr(),
                            line!(),
                            (*b"ClusterBlocksCommand\0").as_ptr()
                        );
                    }
                    i = i.wrapping_add(64i32 as (usize));
                    continue 'loop4;
                }
            } else {
                BrotliFree(m,histograms as (*mut ::std::os::raw::c_void));
                histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand);
                max_num_pairs = brotli_min_size_t(
                                    (64i32 as (usize)).wrapping_mul(num_clusters),
                                    num_clusters.wrapping_div(2i32 as (usize)).wrapping_mul(
                                        num_clusters
                                    )
                                );
                if pairs_capacity < max_num_pairs.wrapping_add(1i32 as (usize)) {
                    BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
                    pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
                    pairs = if max_num_pairs.wrapping_add(1i32 as (usize)) != 0 {
                                BrotliAllocate(
                                    m,
                                    max_num_pairs.wrapping_add(1i32 as (usize)).wrapping_mul(
                                        ::std::mem::size_of::<HistogramPair>()
                                    )
                                ) as (*mut HistogramPair)
                            } else {
                                0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
                            };
                    if !(0i32 == 0) {
                        return;
                    }
                }
                clusters = if num_clusters != 0 {
                               BrotliAllocate(
                                   m,
                                   num_clusters.wrapping_mul(::std::mem::size_of::<u32>())
                               ) as (*mut u32)
                           } else {
                               0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                           };
                if !(0i32 == 0) {
                    return;
                } else {
                    i = 0i32 as (usize);
                    'loop9: loop {
                        if i < num_clusters {
                            *clusters.offset(i as (isize)) = i as (u32);
                            i = i.wrapping_add(1 as (usize));
                            continue 'loop9;
                        } else {
                            break 'loop9;
                        }
                    }
                    num_final_clusters = BrotliHistogramCombineCommand(
                                             all_histograms,
                                             cluster_size,
                                             histogram_symbols,
                                             clusters,
                                             pairs,
                                             num_clusters,
                                             num_blocks,
                                             256i32 as (usize),
                                             max_num_pairs
                                         );
                    BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
                    pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
                    BrotliFree(m,cluster_size as (*mut ::std::os::raw::c_void));
                    cluster_size = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                    new_index = if num_clusters != 0 {
                                    BrotliAllocate(
                                        m,
                                        num_clusters.wrapping_mul(::std::mem::size_of::<u32>())
                                    ) as (*mut u32)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                };
                    if !(0i32 == 0) {
                        return;
                    } else {
                        i = 0i32 as (usize);
                        'loop12: loop {
                            if i < num_clusters {
                                *new_index.offset(i as (isize)) = kInvalidIndex;
                                i = i.wrapping_add(1 as (usize));
                                continue 'loop12;
                            } else {
                                break 'loop12;
                            }
                        }
                        pos = 0i32 as (usize);
                        let mut next_index : u32 = 0i32 as (u32);
                        i = 0i32 as (usize);
                        'loop14: loop {
                            if i < num_blocks {
                                let mut histo : HistogramCommand;
                                let mut j : usize;
                                let mut best_out : u32;
                                let mut best_bits : f64;
                                HistogramClearCommand(&mut histo as (*mut HistogramCommand));
                                j = 0i32 as (usize);
                                'loop38: loop {
                                    if j < *block_lengths.offset(i as (isize)) as (usize) {
                                        HistogramAddCommand(
                                            &mut histo as (*mut HistogramCommand),
                                            *data.offset(
                                                 {
                                                     let _old = pos;
                                                     pos = pos.wrapping_add(1 as (usize));
                                                     _old
                                                 } as (isize)
                                             ) as (usize)
                                        );
                                        j = j.wrapping_add(1 as (usize));
                                        continue 'loop38;
                                    } else {
                                        break 'loop38;
                                    }
                                }
                                best_out = if i == 0i32 as (usize) {
                                               *histogram_symbols.offset(0i32 as (isize))
                                           } else {
                                               *histogram_symbols.offset(
                                                    i.wrapping_sub(1i32 as (usize)) as (isize)
                                                )
                                           };
                                best_bits = BrotliHistogramBitCostDistanceCommand(
                                                &mut histo as (*mut HistogramCommand) as (*const HistogramCommand),
                                                &mut *all_histograms.offset(
                                                          best_out as (isize)
                                                      ) as (*mut HistogramCommand) as (*const HistogramCommand)
                                            );
                                j = 0i32 as (usize);
                                'loop40: loop {
                                    if j < num_final_clusters {
                                        let cur_bits
                                            : f64
                                            = BrotliHistogramBitCostDistanceCommand(
                                                  &mut histo as (*mut HistogramCommand) as (*const HistogramCommand),
                                                  &mut *all_histograms.offset(
                                                            *clusters.offset(
                                                                 j as (isize)
                                                             ) as (isize)
                                                        ) as (*mut HistogramCommand) as (*const HistogramCommand)
                                              );
                                        if cur_bits < best_bits {
                                            best_bits = cur_bits;
                                            best_out = *clusters.offset(j as (isize));
                                        }
                                        j = j.wrapping_add(1 as (usize));
                                        continue 'loop40;
                                    } else {
                                        break 'loop40;
                                    }
                                }
                                *histogram_symbols.offset(i as (isize)) = best_out;
                                if *new_index.offset(best_out as (isize)) == kInvalidIndex {
                                    *new_index.offset(best_out as (isize)) = {
                                                                                 let _old
                                                                                     = next_index;
                                                                                 next_index = next_index.wrapping_add(
                                                                                                  1 as (u32)
                                                                                              );
                                                                                 _old
                                                                             };
                                }
                                i = i.wrapping_add(1 as (usize));
                                continue 'loop14;
                            } else {
                                break 'loop14;
                            }
                        }
                        BrotliFree(m,clusters as (*mut ::std::os::raw::c_void));
                        clusters = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                        BrotliFree(m,all_histograms as (*mut ::std::os::raw::c_void));
                        all_histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand);
                        if (*split).types_alloc_size < num_blocks {
                            let mut _new_size
                                : usize
                                = if (*split).types_alloc_size == 0i32 as (usize) {
                                      num_blocks
                                  } else {
                                      (*split).types_alloc_size
                                  };
                            let mut new_array : *mut u8;
                            'loop17: loop {
                                if _new_size < num_blocks {
                                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                                    continue 'loop17;
                                } else {
                                    break 'loop17;
                                }
                            }
                            new_array = if _new_size != 0 {
                                            BrotliAllocate(
                                                m,
                                                _new_size.wrapping_mul(::std::mem::size_of::<u8>())
                                            ) as (*mut u8)
                                        } else {
                                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                                        };
                            if !!(0i32 == 0) && ((*split).types_alloc_size != 0i32 as (usize)) {
                                memcpy(
                                    new_array as (*mut ::std::os::raw::c_void),
                                    (*split).types as (*const ::std::os::raw::c_void),
                                    (*split).types_alloc_size.wrapping_mul(
                                        ::std::mem::size_of::<u8>()
                                    )
                                );
                            }
                            BrotliFree(m,(*split).types as (*mut ::std::os::raw::c_void));
                            (*split).types = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                            (*split).types = new_array;
                            (*split).types_alloc_size = _new_size;
                        }
                        if (*split).lengths_alloc_size < num_blocks {
                            let mut _new_size
                                : usize
                                = if (*split).lengths_alloc_size == 0i32 as (usize) {
                                      num_blocks
                                  } else {
                                      (*split).lengths_alloc_size
                                  };
                            let mut new_array : *mut u32;
                            'loop23: loop {
                                if _new_size < num_blocks {
                                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                                    continue 'loop23;
                                } else {
                                    break 'loop23;
                                }
                            }
                            new_array = if _new_size != 0 {
                                            BrotliAllocate(
                                                m,
                                                _new_size.wrapping_mul(::std::mem::size_of::<u32>())
                                            ) as (*mut u32)
                                        } else {
                                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                        };
                            if !!(0i32 == 0) && ((*split).lengths_alloc_size != 0i32 as (usize)) {
                                memcpy(
                                    new_array as (*mut ::std::os::raw::c_void),
                                    (*split).lengths as (*const ::std::os::raw::c_void),
                                    (*split).lengths_alloc_size.wrapping_mul(
                                        ::std::mem::size_of::<u32>()
                                    )
                                );
                            }
                            BrotliFree(m,(*split).lengths as (*mut ::std::os::raw::c_void));
                            (*split).lengths = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                            (*split).lengths = new_array;
                            (*split).lengths_alloc_size = _new_size;
                        }
                        if !(0i32 == 0) {
                            return;
                        } else {
                            let mut cur_length : u32 = 0i32 as (u32);
                            let mut block_idx : usize = 0i32 as (usize);
                            let mut max_type : u8 = 0i32 as (u8);
                            i = 0i32 as (usize);
                            'loop29: loop {
                                if i < num_blocks {
                                    cur_length = cur_length.wrapping_add(
                                                     *block_lengths.offset(i as (isize))
                                                 );
                                    if i.wrapping_add(
                                           1i32 as (usize)
                                       ) == num_blocks || *histogram_symbols.offset(
                                                               i as (isize)
                                                           ) != *histogram_symbols.offset(
                                                                     i.wrapping_add(
                                                                         1i32 as (usize)
                                                                     ) as (isize)
                                                                 ) {
                                        let id
                                            : u8
                                            = *new_index.offset(
                                                   *histogram_symbols.offset(
                                                        i as (isize)
                                                    ) as (isize)
                                               ) as (u8);
                                        *(*split).types.offset(block_idx as (isize)) = id;
                                        *(*split).lengths.offset(block_idx as (isize)) = cur_length;
                                        max_type = brotli_max_uint8_t(max_type,id);
                                        cur_length = 0i32 as (u32);
                                        block_idx = block_idx.wrapping_add(1 as (usize));
                                    }
                                    i = i.wrapping_add(1 as (usize));
                                    continue 'loop29;
                                } else {
                                    break 'loop29;
                                }
                            }
                            (*split).num_blocks = block_idx;
                            (*split).num_types = (max_type as (usize)).wrapping_add(
                                                     1i32 as (usize)
                                                 );
                            BrotliFree(m,new_index as (*mut ::std::os::raw::c_void));
                            new_index = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                            BrotliFree(m,block_lengths as (*mut ::std::os::raw::c_void));
                            block_lengths = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                            BrotliFree(m,histogram_symbols as (*mut ::std::os::raw::c_void));
                            histogram_symbols = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                        }
                    }
                }
            }
        }
    }
}

unsafe extern fn SplitByteVectorCommand(
    mut m : *mut MemoryManager,
    mut data : *const u16,
    length : usize,
    literals_per_histogram : usize,
    max_histograms : usize,
    sampling_stride_length : usize,
    block_switch_cost : f64,
    mut params : *const BrotliEncoderParams,
    mut split : *mut BlockSplit
) {
    let data_size : usize = HistogramDataSizeCommand();
    let mut num_histograms
        : usize
        = length.wrapping_div(literals_per_histogram).wrapping_add(
              1i32 as (usize)
          );
    let mut histograms : *mut HistogramCommand;
    if num_histograms > max_histograms {
        num_histograms = max_histograms;
    }
    if length == 0i32 as (usize) {
        (*split).num_types = 1i32 as (usize);
    } else if length < kMinLengthForBlockSplitting {
        if (*split).types_alloc_size < (*split).num_blocks.wrapping_add(
                                           1i32 as (usize)
                                       ) {
            let mut _new_size
                : usize
                = if (*split).types_alloc_size == 0i32 as (usize) {
                      (*split).num_blocks.wrapping_add(1i32 as (usize))
                  } else {
                      (*split).types_alloc_size
                  };
            let mut new_array : *mut u8;
            'loop17: loop {
                if _new_size < (*split).num_blocks.wrapping_add(1i32 as (usize)) {
                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                    continue 'loop17;
                } else {
                    break 'loop17;
                }
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(::std::mem::size_of::<u8>())
                            ) as (*mut u8)
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                        };
            if !!(0i32 == 0) && ((*split).types_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut ::std::os::raw::c_void),
                    (*split).types as (*const ::std::os::raw::c_void),
                    (*split).types_alloc_size.wrapping_mul(::std::mem::size_of::<u8>())
                );
            }
            BrotliFree(m,(*split).types as (*mut ::std::os::raw::c_void));
            (*split).types = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
            (*split).types = new_array;
            (*split).types_alloc_size = _new_size;
        }
        if (*split).lengths_alloc_size < (*split).num_blocks.wrapping_add(
                                             1i32 as (usize)
                                         ) {
            let mut _new_size
                : usize
                = if (*split).lengths_alloc_size == 0i32 as (usize) {
                      (*split).num_blocks.wrapping_add(1i32 as (usize))
                  } else {
                      (*split).lengths_alloc_size
                  };
            let mut new_array : *mut u32;
            'loop23: loop {
                if _new_size < (*split).num_blocks.wrapping_add(1i32 as (usize)) {
                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                    continue 'loop23;
                } else {
                    break 'loop23;
                }
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(::std::mem::size_of::<u32>())
                            ) as (*mut u32)
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                        };
            if !!(0i32 == 0) && ((*split).lengths_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut ::std::os::raw::c_void),
                    (*split).lengths as (*const ::std::os::raw::c_void),
                    (*split).lengths_alloc_size.wrapping_mul(
                        ::std::mem::size_of::<u32>()
                    )
                );
            }
            BrotliFree(m,(*split).lengths as (*mut ::std::os::raw::c_void));
            (*split).lengths = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            (*split).lengths = new_array;
            (*split).lengths_alloc_size = _new_size;
        }
        if !(0i32 == 0) {
        } else {
            (*split).num_types = 1i32 as (usize);
            *(*split).types.offset(
                 (*split).num_blocks as (isize)
             ) = 0i32 as (u8);
            *(*split).lengths.offset(
                 (*split).num_blocks as (isize)
             ) = length as (u32);
            (*split).num_blocks = (*split).num_blocks.wrapping_add(
                                      1 as (usize)
                                  );
        }
    } else {
        histograms = if num_histograms != 0 {
                         BrotliAllocate(
                             m,
                             num_histograms.wrapping_mul(
                                 ::std::mem::size_of::<HistogramCommand>()
                             )
                         ) as (*mut HistogramCommand)
                     } else {
                         0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand)
                     };
        if !(0i32 == 0) {
        } else {
            InitialEntropyCodesCommand(
                data,
                length,
                sampling_stride_length,
                num_histograms,
                histograms
            );
            RefineEntropyCodesCommand(
                data,
                length,
                sampling_stride_length,
                num_histograms,
                histograms
            );
            let mut block_ids
                : *mut u8
                = if length != 0 {
                      BrotliAllocate(
                          m,
                          length.wrapping_mul(::std::mem::size_of::<u8>())
                      ) as (*mut u8)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                  };
            let mut num_blocks : usize = 0i32 as (usize);
            let bitmaplen
                : usize
                = num_histograms.wrapping_add(7i32 as (usize)) >> 3i32;
            let mut insert_cost
                : *mut f64
                = if data_size.wrapping_mul(num_histograms) != 0 {
                      BrotliAllocate(
                          m,
                          data_size.wrapping_mul(num_histograms).wrapping_mul(
                              ::std::mem::size_of::<f64>()
                          )
                      ) as (*mut f64)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut f64)
                  };
            let mut cost
                : *mut f64
                = if num_histograms != 0 {
                      BrotliAllocate(
                          m,
                          num_histograms.wrapping_mul(::std::mem::size_of::<f64>())
                      ) as (*mut f64)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut f64)
                  };
            let mut switch_signal
                : *mut u8
                = if length.wrapping_mul(bitmaplen) != 0 {
                      BrotliAllocate(
                          m,
                          length.wrapping_mul(bitmaplen).wrapping_mul(
                              ::std::mem::size_of::<u8>()
                          )
                      ) as (*mut u8)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                  };
            let mut new_id
                : *mut u16
                = if num_histograms != 0 {
                      BrotliAllocate(
                          m,
                          num_histograms.wrapping_mul(::std::mem::size_of::<u16>())
                      ) as (*mut u16)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
                  };
            let iters
                : usize
                = (if (*params).quality < 11i32 {
                       3i32
                   } else {
                       10i32
                   }) as (usize);
            let mut i : usize;
            if !(0i32 == 0) {
            } else {
                i = 0i32 as (usize);
                'loop7: loop {
                    if i < iters {
                        num_blocks = FindBlocksCommand(
                                         data,
                                         length,
                                         block_switch_cost,
                                         num_histograms,
                                         histograms as (*const HistogramCommand),
                                         insert_cost,
                                         cost,
                                         switch_signal,
                                         block_ids
                                     );
                        num_histograms = RemapBlockIdsCommand(
                                             block_ids,
                                             length,
                                             new_id,
                                             num_histograms
                                         );
                        BuildBlockHistogramsCommand(
                            data,
                            length,
                            block_ids as (*const u8),
                            num_histograms,
                            histograms
                        );
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop7;
                    } else {
                        break 'loop7;
                    }
                }
                BrotliFree(m,insert_cost as (*mut ::std::os::raw::c_void));
                insert_cost = 0i32 as (*mut ::std::os::raw::c_void) as (*mut f64);
                BrotliFree(m,cost as (*mut ::std::os::raw::c_void));
                cost = 0i32 as (*mut ::std::os::raw::c_void) as (*mut f64);
                BrotliFree(m,switch_signal as (*mut ::std::os::raw::c_void));
                switch_signal = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                BrotliFree(m,new_id as (*mut ::std::os::raw::c_void));
                new_id = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16);
                BrotliFree(m,histograms as (*mut ::std::os::raw::c_void));
                histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand);
                ClusterBlocksCommand(m,data,length,num_blocks,block_ids,split);
                if !(0i32 == 0) {
                } else {
                    BrotliFree(m,block_ids as (*mut ::std::os::raw::c_void));
                    block_ids = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                }
            }
        }
    }
}

unsafe extern fn HistogramDataSizeDistance() -> usize {
    520i32 as (usize)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramDistance {
    pub data_ : [u32; 520],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

unsafe extern fn HistogramClearDistance(
    mut self : *mut HistogramDistance
) {
    memset(
        (*self).data_.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 520]>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn ClearHistogramsDistance(
    mut array : *mut HistogramDistance, mut length : usize
) {
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < length {
            HistogramClearDistance(array.offset(i as (isize)));
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn HistogramAddVectorDistance(
    mut self : *mut HistogramDistance,
    mut p : *const u16,
    mut n : usize
) {
    (*self).total_count_ = (*self).total_count_.wrapping_add(n);
    n = n.wrapping_add(1i32 as (usize));
    'loop1: loop {
        if {
               n = n.wrapping_sub(1 as (usize));
               n
           } != 0 {
            {
                let _rhs = 1;
                let _lhs
                    = &mut (*self).data_[
                               *{
                                    let _old = p;
                                    p = p.offset(1 as (isize));
                                    _old
                                } as (usize)
                           ];
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn InitialEntropyCodesDistance(
    mut data : *const u16,
    mut length : usize,
    mut stride : usize,
    mut num_histograms : usize,
    mut histograms : *mut HistogramDistance
) {
    let mut seed : u32 = 7i32 as (u32);
    let mut block_length : usize = length.wrapping_div(num_histograms);
    let mut i : usize;
    ClearHistogramsDistance(histograms,num_histograms);
    i = 0i32 as (usize);
    'loop1: loop {
        if i < num_histograms {
            let mut pos
                : usize
                = length.wrapping_mul(i).wrapping_div(num_histograms);
            if i != 0i32 as (usize) {
                pos = pos.wrapping_add(
                          (MyRand(&mut seed as (*mut u32)) as (usize)).wrapping_rem(
                              block_length
                          )
                      );
            }
            if pos.wrapping_add(stride) >= length {
                pos = length.wrapping_sub(stride).wrapping_sub(1i32 as (usize));
            }
            HistogramAddVectorDistance(
                &mut *histograms.offset(i as (isize)) as (*mut HistogramDistance),
                data.offset(pos as (isize)),
                stride
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn RandomSampleDistance(
    mut seed : *mut u32,
    mut data : *const u16,
    mut length : usize,
    mut stride : usize,
    mut sample : *mut HistogramDistance
) {
    let mut pos : usize = 0i32 as (usize);
    if stride >= length {
        pos = 0i32 as (usize);
        stride = length;
    } else {
        pos = (MyRand(seed) as (usize)).wrapping_rem(
                  length.wrapping_sub(stride).wrapping_add(1i32 as (usize))
              );
    }
    HistogramAddVectorDistance(
        sample,
        data.offset(pos as (isize)),
        stride
    );
}

unsafe extern fn HistogramAddHistogramDistance(
    mut self : *mut HistogramDistance, mut v : *const HistogramDistance
) {
    let mut i : usize;
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               (*v).total_count_
                           );
    i = 0i32 as (usize);
    'loop1: loop {
        if i < 520i32 as (usize) {
            {
                let _rhs = (*v).data_[i];
                let _lhs = &mut (*self).data_[i];
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn RefineEntropyCodesDistance(
    mut data : *const u16,
    mut length : usize,
    mut stride : usize,
    mut num_histograms : usize,
    mut histograms : *mut HistogramDistance
) {
    let mut iters
        : usize
        = kIterMulForRefining.wrapping_mul(length).wrapping_div(
              stride
          ).wrapping_add(
              kMinItersForRefining
          );
    let mut seed : u32 = 7i32 as (u32);
    let mut iter : usize;
    iters = iters.wrapping_add(num_histograms).wrapping_sub(
                1i32 as (usize)
            ).wrapping_div(
                num_histograms
            ).wrapping_mul(
                num_histograms
            );
    iter = 0i32 as (usize);
    'loop1: loop {
        if iter < iters {
            let mut sample : HistogramDistance;
            HistogramClearDistance(&mut sample as (*mut HistogramDistance));
            RandomSampleDistance(
                &mut seed as (*mut u32),
                data,
                length,
                stride,
                &mut sample as (*mut HistogramDistance)
            );
            HistogramAddHistogramDistance(
                &mut *histograms.offset(
                          iter.wrapping_rem(num_histograms) as (isize)
                      ) as (*mut HistogramDistance),
                &mut sample as (*mut HistogramDistance) as (*const HistogramDistance)
            );
            iter = iter.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn FindBlocksDistance(
    mut data : *const u16,
    length : usize,
    block_switch_bitcost : f64,
    num_histograms : usize,
    mut histograms : *const HistogramDistance,
    mut insert_cost : *mut f64,
    mut cost : *mut f64,
    mut switch_signal : *mut u8,
    mut block_id : *mut u8
) -> usize {
    let data_size : usize = HistogramDataSizeDistance();
    let bitmaplen
        : usize
        = num_histograms.wrapping_add(7i32 as (usize)) >> 3i32;
    let mut num_blocks : usize = 1i32 as (usize);
    let mut i : usize;
    let mut j : usize;
    if num_histograms <= 256i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            (*b"num_histograms <= 256\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"FindBlocksDistance\0").as_ptr()
        );
    }
    if num_histograms <= 1i32 as (usize) {
        i = 0i32 as (usize);
        'loop34: loop {
            if i < length {
                *block_id.offset(i as (isize)) = 0i32 as (u8);
                i = i.wrapping_add(1 as (usize));
                continue 'loop34;
            } else {
                break 'loop34;
            }
        }
        1i32 as (usize)
    } else {
        memset(
            insert_cost as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<f64>().wrapping_mul(data_size).wrapping_mul(
                num_histograms
            )
        );
        i = 0i32 as (usize);
        'loop2: loop {
            if i < num_histograms {
                *insert_cost.offset(i as (isize)) = FastLog2(
                                                        (*histograms.offset(
                                                              i as (isize)
                                                          )).total_count_ as (u32) as (usize)
                                                    );
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        i = data_size;
        'loop4: loop {
            if i != 0i32 as (usize) {
                i = i.wrapping_sub(1 as (usize));
                j = 0i32 as (usize);
                'loop28: loop {
                    if j < num_histograms {
                        *insert_cost.offset(
                             i.wrapping_mul(num_histograms).wrapping_add(j) as (isize)
                         ) = *insert_cost.offset(j as (isize)) - BitCost(
                                                                     (*histograms.offset(
                                                                           j as (isize)
                                                                       )).data_[
                                                                         i
                                                                     ] as (usize)
                                                                 );
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop28;
                    } else {
                        continue 'loop4;
                    }
                }
            } else {
                break 'loop4;
            }
        }
        memset(
            cost as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<f64>().wrapping_mul(num_histograms)
        );
        memset(
            switch_signal as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<u8>().wrapping_mul(length).wrapping_mul(
                bitmaplen
            )
        );
        i = 0i32 as (usize);
        'loop6: loop {
            if i < length {
                let byte_ix : usize = i;
                let mut ix : usize = byte_ix.wrapping_mul(bitmaplen);
                let mut insert_cost_ix
                    : usize
                    = (*data.offset(byte_ix as (isize)) as (usize)).wrapping_mul(
                          num_histograms
                      );
                let mut min_cost : f64 = 1e99f64;
                let mut block_switch_cost : f64 = block_switch_bitcost;
                let mut k : usize;
                k = 0i32 as (usize);
                'loop15: loop {
                    if k < num_histograms {
                        {
                            let _rhs
                                = *insert_cost.offset(insert_cost_ix.wrapping_add(k) as (isize));
                            let _lhs = &mut *cost.offset(k as (isize));
                            *_lhs = *_lhs + _rhs;
                        }
                        if *cost.offset(k as (isize)) < min_cost {
                            min_cost = *cost.offset(k as (isize));
                            *block_id.offset(byte_ix as (isize)) = k as (u8);
                        }
                        k = k.wrapping_add(1 as (usize));
                        continue 'loop15;
                    } else {
                        break 'loop15;
                    }
                }
                if byte_ix < 2000i32 as (usize) {
                    block_switch_cost = block_switch_cost * (0.77f64 + 0.07f64 * byte_ix as (f64) / 2000i32 as (f64));
                }
                k = 0i32 as (usize);
                'loop19: loop {
                    if k < num_histograms {
                        {
                            let _rhs = min_cost;
                            let _lhs = &mut *cost.offset(k as (isize));
                            *_lhs = *_lhs - _rhs;
                        }
                        if *cost.offset(k as (isize)) >= block_switch_cost {
                            let mask : u8 = (1u32 << (k & 7i32 as (usize))) as (u8);
                            *cost.offset(k as (isize)) = block_switch_cost;
                            if k >> 3i32 < bitmaplen {
                                0i32;
                            } else {
                                __assert_fail(
                                    (*b"(k >> 3) < bitmaplen\0").as_ptr(),
                                    file!().as_ptr(),
                                    line!(),
                                    (*b"FindBlocksDistance\0").as_ptr()
                                );
                            }
                            {
                                let _rhs = mask;
                                let _lhs
                                    = &mut *switch_signal.offset(
                                                ix.wrapping_add(k >> 3i32) as (isize)
                                            );
                                *_lhs = (*_lhs as (i32) | _rhs as (i32)) as (u8);
                            }
                        }
                        k = k.wrapping_add(1 as (usize));
                        continue 'loop19;
                    } else {
                        break 'loop19;
                    }
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop6;
            } else {
                break 'loop6;
            }
        }
        let mut byte_ix : usize = length.wrapping_sub(1i32 as (usize));
        let mut ix : usize = byte_ix.wrapping_mul(bitmaplen);
        let mut cur_id : u8 = *block_id.offset(byte_ix as (isize));
        'loop8: loop {
            if byte_ix > 0i32 as (usize) {
                let mask : u8 = (1u32 << (cur_id as (i32) & 7i32)) as (u8);
                if cur_id as (usize) >> 3i32 < bitmaplen {
                    0i32;
                } else {
                    __assert_fail(
                        (*b"((size_t)cur_id >> 3) < bitmaplen\0").as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        (*b"FindBlocksDistance\0").as_ptr()
                    );
                }
                byte_ix = byte_ix.wrapping_sub(1 as (usize));
                ix = ix.wrapping_sub(bitmaplen);
                if *switch_signal.offset(
                        ix.wrapping_add((cur_id as (i32) >> 3i32) as (usize)) as (isize)
                    ) as (i32) & mask as (i32) != 0 {
                    if cur_id as (i32) != *block_id.offset(
                                               byte_ix as (isize)
                                           ) as (i32) {
                        cur_id = *block_id.offset(byte_ix as (isize));
                        num_blocks = num_blocks.wrapping_add(1 as (usize));
                    }
                }
                *block_id.offset(byte_ix as (isize)) = cur_id;
                continue 'loop8;
            } else {
                break 'loop8;
            }
        }
        num_blocks
    }
}

unsafe extern fn RemapBlockIdsDistance(
    mut block_ids : *mut u8,
    length : usize,
    mut new_id : *mut u16,
    num_histograms : usize
) -> usize {
    static kInvalidId : u16 = 256i32 as (u16);
    let mut next_id : u16 = 0i32 as (u16);
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < num_histograms {
            *new_id.offset(i as (isize)) = kInvalidId;
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    i = 0i32 as (usize);
    'loop3: loop {
        if i < length {
            if *block_ids.offset(i as (isize)) as (usize) < num_histograms {
                0i32;
            } else {
                __assert_fail(
                    (*b"block_ids[i] < num_histograms\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"RemapBlockIdsDistance\0").as_ptr()
                );
            }
            if *new_id.offset(
                    *block_ids.offset(i as (isize)) as (isize)
                ) as (i32) == kInvalidId as (i32) {
                *new_id.offset(*block_ids.offset(i as (isize)) as (isize)) = {
                                                                                 let _old = next_id;
                                                                                 next_id = (next_id as (i32) + 1) as (u16);
                                                                                 _old
                                                                             };
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    i = 0i32 as (usize);
    'loop5: loop {
        if i < length {
            *block_ids.offset(i as (isize)) = *new_id.offset(
                                                   *block_ids.offset(i as (isize)) as (isize)
                                               ) as (u8);
            if *block_ids.offset(i as (isize)) as (usize) < num_histograms {
                0i32;
            } else {
                __assert_fail(
                    (*b"block_ids[i] < num_histograms\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"RemapBlockIdsDistance\0").as_ptr()
                );
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop5;
        } else {
            break 'loop5;
        }
    }
    if next_id as (usize) <= num_histograms {
        0i32;
    } else {
        __assert_fail(
            (*b"next_id <= num_histograms\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"RemapBlockIdsDistance\0").as_ptr()
        );
    }
    next_id as (usize)
}

unsafe extern fn HistogramAddDistance(
    mut self : *mut HistogramDistance, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut (*self).data_[val];
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn BuildBlockHistogramsDistance(
    mut data : *const u16,
    length : usize,
    mut block_ids : *const u8,
    num_histograms : usize,
    mut histograms : *mut HistogramDistance
) {
    let mut i : usize;
    ClearHistogramsDistance(histograms,num_histograms);
    i = 0i32 as (usize);
    'loop1: loop {
        if i < length {
            HistogramAddDistance(
                &mut *histograms.offset(
                          *block_ids.offset(i as (isize)) as (isize)
                      ) as (*mut HistogramDistance),
                *data.offset(i as (isize)) as (usize)
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn ClusterBlocksDistance(
    mut m : *mut MemoryManager,
    mut data : *const u16,
    length : usize,
    num_blocks : usize,
    mut block_ids : *mut u8,
    mut split : *mut BlockSplit
) {
    let mut histogram_symbols
        : *mut u32
        = if num_blocks != 0 {
              BrotliAllocate(
                  m,
                  num_blocks.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut block_lengths
        : *mut u32
        = if num_blocks != 0 {
              BrotliAllocate(
                  m,
                  num_blocks.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let expected_num_clusters
        : usize
        = (16i32 as (usize)).wrapping_mul(
              num_blocks.wrapping_add(64i32 as (usize)).wrapping_sub(
                  1i32 as (usize)
              )
          ).wrapping_div(
              64i32 as (usize)
          );
    let mut all_histograms_size : usize = 0i32 as (usize);
    let mut all_histograms_capacity : usize = expected_num_clusters;
    let mut all_histograms
        : *mut HistogramDistance
        = if all_histograms_capacity != 0 {
              BrotliAllocate(
                  m,
                  all_histograms_capacity.wrapping_mul(
                      ::std::mem::size_of::<HistogramDistance>()
                  )
              ) as (*mut HistogramDistance)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance)
          };
    let mut cluster_size_size : usize = 0i32 as (usize);
    let mut cluster_size_capacity : usize = expected_num_clusters;
    let mut cluster_size
        : *mut u32
        = if cluster_size_capacity != 0 {
              BrotliAllocate(
                  m,
                  cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut num_clusters : usize = 0i32 as (usize);
    let mut histograms
        : *mut HistogramDistance
        = if brotli_min_size_t(num_blocks,64i32 as (usize)) != 0 {
              BrotliAllocate(
                  m,
                  brotli_min_size_t(num_blocks,64i32 as (usize)).wrapping_mul(
                      ::std::mem::size_of::<HistogramDistance>()
                  )
              ) as (*mut HistogramDistance)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance)
          };
    let mut max_num_pairs : usize = (64i32 * 64i32 / 2i32) as (usize);
    let mut pairs_capacity
        : usize
        = max_num_pairs.wrapping_add(1i32 as (usize));
    let mut pairs
        : *mut HistogramPair
        = if pairs_capacity != 0 {
              BrotliAllocate(
                  m,
                  pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>())
              ) as (*mut HistogramPair)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
          };
    let mut pos : usize = 0i32 as (usize);
    let mut clusters : *mut u32;
    let mut num_final_clusters : usize;
    static kInvalidIndex : u32 = !(0i32 as (u32));
    let mut new_index : *mut u32;
    let mut i : usize;
    let mut sizes
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut new_clusters
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut symbols
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut remap
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    if !(0i32 == 0) {
    } else {
        memset(
            block_lengths as (*mut ::std::os::raw::c_void),
            0i32,
            num_blocks.wrapping_mul(::std::mem::size_of::<u32>())
        );
        let mut block_idx : usize = 0i32 as (usize);
        i = 0i32 as (usize);
        'loop2: loop {
            if i < length {
                if block_idx < num_blocks {
                    0i32;
                } else {
                    __assert_fail(
                        (*b"block_idx < num_blocks\0").as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        (*b"ClusterBlocksDistance\0").as_ptr()
                    );
                }
                {
                    let _rhs = 1;
                    let _lhs = &mut *block_lengths.offset(block_idx as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                }
                if i.wrapping_add(1i32 as (usize)) == length || *block_ids.offset(
                                                                     i as (isize)
                                                                 ) as (i32) != *block_ids.offset(
                                                                                    i.wrapping_add(
                                                                                        1i32 as (usize)
                                                                                    ) as (isize)
                                                                                ) as (i32) {
                    block_idx = block_idx.wrapping_add(1 as (usize));
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        if block_idx == num_blocks {
            0i32;
        } else {
            __assert_fail(
                (*b"block_idx == num_blocks\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"ClusterBlocksDistance\0").as_ptr()
            );
        }
        i = 0i32 as (usize);
        'loop4: loop {
            if i < num_blocks {
                let num_to_combine
                    : usize
                    = brotli_min_size_t(num_blocks.wrapping_sub(i),64i32 as (usize));
                let mut num_new_clusters : usize;
                let mut j : usize;
                j = 0i32 as (usize);
                'loop57: loop {
                    if j < num_to_combine {
                        let mut k : usize;
                        HistogramClearDistance(
                            &mut *histograms.offset(j as (isize)) as (*mut HistogramDistance)
                        );
                        k = 0i32 as (usize);
                        'loop85: loop {
                            if k < *block_lengths.offset(
                                        i.wrapping_add(j) as (isize)
                                    ) as (usize) {
                                HistogramAddDistance(
                                    &mut *histograms.offset(
                                              j as (isize)
                                          ) as (*mut HistogramDistance),
                                    *data.offset(
                                         {
                                             let _old = pos;
                                             pos = pos.wrapping_add(1 as (usize));
                                             _old
                                         } as (isize)
                                     ) as (usize)
                                );
                                k = k.wrapping_add(1 as (usize));
                                continue 'loop85;
                            } else {
                                break 'loop85;
                            }
                        }
                        (*histograms.offset(
                              j as (isize)
                          )).bit_cost_ = BrotliPopulationCostDistance(
                                             &mut *histograms.offset(
                                                       j as (isize)
                                                   ) as (*mut HistogramDistance) as (*const HistogramDistance)
                                         );
                        new_clusters[j] = j as (u32);
                        symbols[j] = j as (u32);
                        sizes[j] = 1i32 as (u32);
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop57;
                    } else {
                        break 'loop57;
                    }
                }
                num_new_clusters = BrotliHistogramCombineDistance(
                                       histograms,
                                       sizes.as_mut_ptr(),
                                       symbols.as_mut_ptr(),
                                       new_clusters.as_mut_ptr(),
                                       pairs,
                                       num_to_combine,
                                       num_to_combine,
                                       64i32 as (usize),
                                       max_num_pairs
                                   );
                if all_histograms_capacity < all_histograms_size.wrapping_add(
                                                 num_new_clusters
                                             ) {
                    let mut _new_size
                        : usize
                        = if all_histograms_capacity == 0i32 as (usize) {
                              all_histograms_size.wrapping_add(num_new_clusters)
                          } else {
                              all_histograms_capacity
                          };
                    let mut new_array : *mut HistogramDistance;
                    'loop60: loop {
                        if _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
                            _new_size = _new_size.wrapping_mul(2i32 as (usize));
                            continue 'loop60;
                        } else {
                            break 'loop60;
                        }
                    }
                    new_array = if _new_size != 0 {
                                    BrotliAllocate(
                                        m,
                                        _new_size.wrapping_mul(
                                            ::std::mem::size_of::<HistogramDistance>()
                                        )
                                    ) as (*mut HistogramDistance)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance)
                                };
                    if !!(0i32 == 0) && (all_histograms_capacity != 0i32 as (usize)) {
                        memcpy(
                            new_array as (*mut ::std::os::raw::c_void),
                            all_histograms as (*const ::std::os::raw::c_void),
                            all_histograms_capacity.wrapping_mul(
                                ::std::mem::size_of::<HistogramDistance>()
                            )
                        );
                    }
                    BrotliFree(m,all_histograms as (*mut ::std::os::raw::c_void));
                    all_histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance);
                    all_histograms = new_array;
                    all_histograms_capacity = _new_size;
                }
                if cluster_size_capacity < cluster_size_size.wrapping_add(
                                               num_new_clusters
                                           ) {
                    let mut _new_size
                        : usize
                        = if cluster_size_capacity == 0i32 as (usize) {
                              cluster_size_size.wrapping_add(num_new_clusters)
                          } else {
                              cluster_size_capacity
                          };
                    let mut new_array : *mut u32;
                    'loop66: loop {
                        if _new_size < cluster_size_size.wrapping_add(num_new_clusters) {
                            _new_size = _new_size.wrapping_mul(2i32 as (usize));
                            continue 'loop66;
                        } else {
                            break 'loop66;
                        }
                    }
                    new_array = if _new_size != 0 {
                                    BrotliAllocate(
                                        m,
                                        _new_size.wrapping_mul(::std::mem::size_of::<u32>())
                                    ) as (*mut u32)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                };
                    if !!(0i32 == 0) && (cluster_size_capacity != 0i32 as (usize)) {
                        memcpy(
                            new_array as (*mut ::std::os::raw::c_void),
                            cluster_size as (*const ::std::os::raw::c_void),
                            cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>())
                        );
                    }
                    BrotliFree(m,cluster_size as (*mut ::std::os::raw::c_void));
                    cluster_size = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                    cluster_size = new_array;
                    cluster_size_capacity = _new_size;
                }
                if !(0i32 == 0) {
                    break 'loop4;
                } else {
                    j = 0i32 as (usize);
                    'loop72: loop {
                        if j < num_new_clusters {
                            *all_histograms.offset(
                                 {
                                     let _old = all_histograms_size;
                                     all_histograms_size = all_histograms_size.wrapping_add(
                                                               1 as (usize)
                                                           );
                                     _old
                                 } as (isize)
                             ) = *histograms.offset(new_clusters[j] as (isize));
                            *cluster_size.offset(
                                 {
                                     let _old = cluster_size_size;
                                     cluster_size_size = cluster_size_size.wrapping_add(
                                                             1 as (usize)
                                                         );
                                     _old
                                 } as (isize)
                             ) = sizes[new_clusters[j] as (usize)];
                            remap[new_clusters[j] as (usize)] = j as (u32);
                            j = j.wrapping_add(1 as (usize));
                            continue 'loop72;
                        } else {
                            break 'loop72;
                        }
                    }
                    j = 0i32 as (usize);
                    'loop74: loop {
                        if j < num_to_combine {
                            *histogram_symbols.offset(
                                 i.wrapping_add(j) as (isize)
                             ) = (num_clusters as (u32)).wrapping_add(
                                     remap[symbols[j] as (usize)]
                                 );
                            j = j.wrapping_add(1 as (usize));
                            continue 'loop74;
                        } else {
                            break 'loop74;
                        }
                    }
                    num_clusters = num_clusters.wrapping_add(num_new_clusters);
                    if num_clusters == cluster_size_size {
                        0i32;
                    } else {
                        __assert_fail(
                            (*b"num_clusters == cluster_size_size\0").as_ptr(),
                            file!().as_ptr(),
                            line!(),
                            (*b"ClusterBlocksDistance\0").as_ptr()
                        );
                    }
                    if num_clusters == all_histograms_size {
                        0i32;
                    } else {
                        __assert_fail(
                            (*b"num_clusters == all_histograms_size\0").as_ptr(),
                            file!().as_ptr(),
                            line!(),
                            (*b"ClusterBlocksDistance\0").as_ptr()
                        );
                    }
                    i = i.wrapping_add(64i32 as (usize));
                    continue 'loop4;
                }
            } else {
                BrotliFree(m,histograms as (*mut ::std::os::raw::c_void));
                histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance);
                max_num_pairs = brotli_min_size_t(
                                    (64i32 as (usize)).wrapping_mul(num_clusters),
                                    num_clusters.wrapping_div(2i32 as (usize)).wrapping_mul(
                                        num_clusters
                                    )
                                );
                if pairs_capacity < max_num_pairs.wrapping_add(1i32 as (usize)) {
                    BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
                    pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
                    pairs = if max_num_pairs.wrapping_add(1i32 as (usize)) != 0 {
                                BrotliAllocate(
                                    m,
                                    max_num_pairs.wrapping_add(1i32 as (usize)).wrapping_mul(
                                        ::std::mem::size_of::<HistogramPair>()
                                    )
                                ) as (*mut HistogramPair)
                            } else {
                                0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
                            };
                    if !(0i32 == 0) {
                        return;
                    }
                }
                clusters = if num_clusters != 0 {
                               BrotliAllocate(
                                   m,
                                   num_clusters.wrapping_mul(::std::mem::size_of::<u32>())
                               ) as (*mut u32)
                           } else {
                               0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                           };
                if !(0i32 == 0) {
                    return;
                } else {
                    i = 0i32 as (usize);
                    'loop9: loop {
                        if i < num_clusters {
                            *clusters.offset(i as (isize)) = i as (u32);
                            i = i.wrapping_add(1 as (usize));
                            continue 'loop9;
                        } else {
                            break 'loop9;
                        }
                    }
                    num_final_clusters = BrotliHistogramCombineDistance(
                                             all_histograms,
                                             cluster_size,
                                             histogram_symbols,
                                             clusters,
                                             pairs,
                                             num_clusters,
                                             num_blocks,
                                             256i32 as (usize),
                                             max_num_pairs
                                         );
                    BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
                    pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
                    BrotliFree(m,cluster_size as (*mut ::std::os::raw::c_void));
                    cluster_size = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                    new_index = if num_clusters != 0 {
                                    BrotliAllocate(
                                        m,
                                        num_clusters.wrapping_mul(::std::mem::size_of::<u32>())
                                    ) as (*mut u32)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                };
                    if !(0i32 == 0) {
                        return;
                    } else {
                        i = 0i32 as (usize);
                        'loop12: loop {
                            if i < num_clusters {
                                *new_index.offset(i as (isize)) = kInvalidIndex;
                                i = i.wrapping_add(1 as (usize));
                                continue 'loop12;
                            } else {
                                break 'loop12;
                            }
                        }
                        pos = 0i32 as (usize);
                        let mut next_index : u32 = 0i32 as (u32);
                        i = 0i32 as (usize);
                        'loop14: loop {
                            if i < num_blocks {
                                let mut histo : HistogramDistance;
                                let mut j : usize;
                                let mut best_out : u32;
                                let mut best_bits : f64;
                                HistogramClearDistance(&mut histo as (*mut HistogramDistance));
                                j = 0i32 as (usize);
                                'loop38: loop {
                                    if j < *block_lengths.offset(i as (isize)) as (usize) {
                                        HistogramAddDistance(
                                            &mut histo as (*mut HistogramDistance),
                                            *data.offset(
                                                 {
                                                     let _old = pos;
                                                     pos = pos.wrapping_add(1 as (usize));
                                                     _old
                                                 } as (isize)
                                             ) as (usize)
                                        );
                                        j = j.wrapping_add(1 as (usize));
                                        continue 'loop38;
                                    } else {
                                        break 'loop38;
                                    }
                                }
                                best_out = if i == 0i32 as (usize) {
                                               *histogram_symbols.offset(0i32 as (isize))
                                           } else {
                                               *histogram_symbols.offset(
                                                    i.wrapping_sub(1i32 as (usize)) as (isize)
                                                )
                                           };
                                best_bits = BrotliHistogramBitCostDistanceDistance(
                                                &mut histo as (*mut HistogramDistance) as (*const HistogramDistance),
                                                &mut *all_histograms.offset(
                                                          best_out as (isize)
                                                      ) as (*mut HistogramDistance) as (*const HistogramDistance)
                                            );
                                j = 0i32 as (usize);
                                'loop40: loop {
                                    if j < num_final_clusters {
                                        let cur_bits
                                            : f64
                                            = BrotliHistogramBitCostDistanceDistance(
                                                  &mut histo as (*mut HistogramDistance) as (*const HistogramDistance),
                                                  &mut *all_histograms.offset(
                                                            *clusters.offset(
                                                                 j as (isize)
                                                             ) as (isize)
                                                        ) as (*mut HistogramDistance) as (*const HistogramDistance)
                                              );
                                        if cur_bits < best_bits {
                                            best_bits = cur_bits;
                                            best_out = *clusters.offset(j as (isize));
                                        }
                                        j = j.wrapping_add(1 as (usize));
                                        continue 'loop40;
                                    } else {
                                        break 'loop40;
                                    }
                                }
                                *histogram_symbols.offset(i as (isize)) = best_out;
                                if *new_index.offset(best_out as (isize)) == kInvalidIndex {
                                    *new_index.offset(best_out as (isize)) = {
                                                                                 let _old
                                                                                     = next_index;
                                                                                 next_index = next_index.wrapping_add(
                                                                                                  1 as (u32)
                                                                                              );
                                                                                 _old
                                                                             };
                                }
                                i = i.wrapping_add(1 as (usize));
                                continue 'loop14;
                            } else {
                                break 'loop14;
                            }
                        }
                        BrotliFree(m,clusters as (*mut ::std::os::raw::c_void));
                        clusters = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                        BrotliFree(m,all_histograms as (*mut ::std::os::raw::c_void));
                        all_histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance);
                        if (*split).types_alloc_size < num_blocks {
                            let mut _new_size
                                : usize
                                = if (*split).types_alloc_size == 0i32 as (usize) {
                                      num_blocks
                                  } else {
                                      (*split).types_alloc_size
                                  };
                            let mut new_array : *mut u8;
                            'loop17: loop {
                                if _new_size < num_blocks {
                                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                                    continue 'loop17;
                                } else {
                                    break 'loop17;
                                }
                            }
                            new_array = if _new_size != 0 {
                                            BrotliAllocate(
                                                m,
                                                _new_size.wrapping_mul(::std::mem::size_of::<u8>())
                                            ) as (*mut u8)
                                        } else {
                                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                                        };
                            if !!(0i32 == 0) && ((*split).types_alloc_size != 0i32 as (usize)) {
                                memcpy(
                                    new_array as (*mut ::std::os::raw::c_void),
                                    (*split).types as (*const ::std::os::raw::c_void),
                                    (*split).types_alloc_size.wrapping_mul(
                                        ::std::mem::size_of::<u8>()
                                    )
                                );
                            }
                            BrotliFree(m,(*split).types as (*mut ::std::os::raw::c_void));
                            (*split).types = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                            (*split).types = new_array;
                            (*split).types_alloc_size = _new_size;
                        }
                        if (*split).lengths_alloc_size < num_blocks {
                            let mut _new_size
                                : usize
                                = if (*split).lengths_alloc_size == 0i32 as (usize) {
                                      num_blocks
                                  } else {
                                      (*split).lengths_alloc_size
                                  };
                            let mut new_array : *mut u32;
                            'loop23: loop {
                                if _new_size < num_blocks {
                                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                                    continue 'loop23;
                                } else {
                                    break 'loop23;
                                }
                            }
                            new_array = if _new_size != 0 {
                                            BrotliAllocate(
                                                m,
                                                _new_size.wrapping_mul(::std::mem::size_of::<u32>())
                                            ) as (*mut u32)
                                        } else {
                                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                        };
                            if !!(0i32 == 0) && ((*split).lengths_alloc_size != 0i32 as (usize)) {
                                memcpy(
                                    new_array as (*mut ::std::os::raw::c_void),
                                    (*split).lengths as (*const ::std::os::raw::c_void),
                                    (*split).lengths_alloc_size.wrapping_mul(
                                        ::std::mem::size_of::<u32>()
                                    )
                                );
                            }
                            BrotliFree(m,(*split).lengths as (*mut ::std::os::raw::c_void));
                            (*split).lengths = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                            (*split).lengths = new_array;
                            (*split).lengths_alloc_size = _new_size;
                        }
                        if !(0i32 == 0) {
                            return;
                        } else {
                            let mut cur_length : u32 = 0i32 as (u32);
                            let mut block_idx : usize = 0i32 as (usize);
                            let mut max_type : u8 = 0i32 as (u8);
                            i = 0i32 as (usize);
                            'loop29: loop {
                                if i < num_blocks {
                                    cur_length = cur_length.wrapping_add(
                                                     *block_lengths.offset(i as (isize))
                                                 );
                                    if i.wrapping_add(
                                           1i32 as (usize)
                                       ) == num_blocks || *histogram_symbols.offset(
                                                               i as (isize)
                                                           ) != *histogram_symbols.offset(
                                                                     i.wrapping_add(
                                                                         1i32 as (usize)
                                                                     ) as (isize)
                                                                 ) {
                                        let id
                                            : u8
                                            = *new_index.offset(
                                                   *histogram_symbols.offset(
                                                        i as (isize)
                                                    ) as (isize)
                                               ) as (u8);
                                        *(*split).types.offset(block_idx as (isize)) = id;
                                        *(*split).lengths.offset(block_idx as (isize)) = cur_length;
                                        max_type = brotli_max_uint8_t(max_type,id);
                                        cur_length = 0i32 as (u32);
                                        block_idx = block_idx.wrapping_add(1 as (usize));
                                    }
                                    i = i.wrapping_add(1 as (usize));
                                    continue 'loop29;
                                } else {
                                    break 'loop29;
                                }
                            }
                            (*split).num_blocks = block_idx;
                            (*split).num_types = (max_type as (usize)).wrapping_add(
                                                     1i32 as (usize)
                                                 );
                            BrotliFree(m,new_index as (*mut ::std::os::raw::c_void));
                            new_index = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                            BrotliFree(m,block_lengths as (*mut ::std::os::raw::c_void));
                            block_lengths = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                            BrotliFree(m,histogram_symbols as (*mut ::std::os::raw::c_void));
                            histogram_symbols = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                        }
                    }
                }
            }
        }
    }
}

unsafe extern fn SplitByteVectorDistance(
    mut m : *mut MemoryManager,
    mut data : *const u16,
    length : usize,
    literals_per_histogram : usize,
    max_histograms : usize,
    sampling_stride_length : usize,
    block_switch_cost : f64,
    mut params : *const BrotliEncoderParams,
    mut split : *mut BlockSplit
) {
    let data_size : usize = HistogramDataSizeDistance();
    let mut num_histograms
        : usize
        = length.wrapping_div(literals_per_histogram).wrapping_add(
              1i32 as (usize)
          );
    let mut histograms : *mut HistogramDistance;
    if num_histograms > max_histograms {
        num_histograms = max_histograms;
    }
    if length == 0i32 as (usize) {
        (*split).num_types = 1i32 as (usize);
    } else if length < kMinLengthForBlockSplitting {
        if (*split).types_alloc_size < (*split).num_blocks.wrapping_add(
                                           1i32 as (usize)
                                       ) {
            let mut _new_size
                : usize
                = if (*split).types_alloc_size == 0i32 as (usize) {
                      (*split).num_blocks.wrapping_add(1i32 as (usize))
                  } else {
                      (*split).types_alloc_size
                  };
            let mut new_array : *mut u8;
            'loop17: loop {
                if _new_size < (*split).num_blocks.wrapping_add(1i32 as (usize)) {
                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                    continue 'loop17;
                } else {
                    break 'loop17;
                }
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(::std::mem::size_of::<u8>())
                            ) as (*mut u8)
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                        };
            if !!(0i32 == 0) && ((*split).types_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut ::std::os::raw::c_void),
                    (*split).types as (*const ::std::os::raw::c_void),
                    (*split).types_alloc_size.wrapping_mul(::std::mem::size_of::<u8>())
                );
            }
            BrotliFree(m,(*split).types as (*mut ::std::os::raw::c_void));
            (*split).types = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
            (*split).types = new_array;
            (*split).types_alloc_size = _new_size;
        }
        if (*split).lengths_alloc_size < (*split).num_blocks.wrapping_add(
                                             1i32 as (usize)
                                         ) {
            let mut _new_size
                : usize
                = if (*split).lengths_alloc_size == 0i32 as (usize) {
                      (*split).num_blocks.wrapping_add(1i32 as (usize))
                  } else {
                      (*split).lengths_alloc_size
                  };
            let mut new_array : *mut u32;
            'loop23: loop {
                if _new_size < (*split).num_blocks.wrapping_add(1i32 as (usize)) {
                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                    continue 'loop23;
                } else {
                    break 'loop23;
                }
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(::std::mem::size_of::<u32>())
                            ) as (*mut u32)
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                        };
            if !!(0i32 == 0) && ((*split).lengths_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut ::std::os::raw::c_void),
                    (*split).lengths as (*const ::std::os::raw::c_void),
                    (*split).lengths_alloc_size.wrapping_mul(
                        ::std::mem::size_of::<u32>()
                    )
                );
            }
            BrotliFree(m,(*split).lengths as (*mut ::std::os::raw::c_void));
            (*split).lengths = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            (*split).lengths = new_array;
            (*split).lengths_alloc_size = _new_size;
        }
        if !(0i32 == 0) {
        } else {
            (*split).num_types = 1i32 as (usize);
            *(*split).types.offset(
                 (*split).num_blocks as (isize)
             ) = 0i32 as (u8);
            *(*split).lengths.offset(
                 (*split).num_blocks as (isize)
             ) = length as (u32);
            (*split).num_blocks = (*split).num_blocks.wrapping_add(
                                      1 as (usize)
                                  );
        }
    } else {
        histograms = if num_histograms != 0 {
                         BrotliAllocate(
                             m,
                             num_histograms.wrapping_mul(
                                 ::std::mem::size_of::<HistogramDistance>()
                             )
                         ) as (*mut HistogramDistance)
                     } else {
                         0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance)
                     };
        if !(0i32 == 0) {
        } else {
            InitialEntropyCodesDistance(
                data,
                length,
                sampling_stride_length,
                num_histograms,
                histograms
            );
            RefineEntropyCodesDistance(
                data,
                length,
                sampling_stride_length,
                num_histograms,
                histograms
            );
            let mut block_ids
                : *mut u8
                = if length != 0 {
                      BrotliAllocate(
                          m,
                          length.wrapping_mul(::std::mem::size_of::<u8>())
                      ) as (*mut u8)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                  };
            let mut num_blocks : usize = 0i32 as (usize);
            let bitmaplen
                : usize
                = num_histograms.wrapping_add(7i32 as (usize)) >> 3i32;
            let mut insert_cost
                : *mut f64
                = if data_size.wrapping_mul(num_histograms) != 0 {
                      BrotliAllocate(
                          m,
                          data_size.wrapping_mul(num_histograms).wrapping_mul(
                              ::std::mem::size_of::<f64>()
                          )
                      ) as (*mut f64)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut f64)
                  };
            let mut cost
                : *mut f64
                = if num_histograms != 0 {
                      BrotliAllocate(
                          m,
                          num_histograms.wrapping_mul(::std::mem::size_of::<f64>())
                      ) as (*mut f64)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut f64)
                  };
            let mut switch_signal
                : *mut u8
                = if length.wrapping_mul(bitmaplen) != 0 {
                      BrotliAllocate(
                          m,
                          length.wrapping_mul(bitmaplen).wrapping_mul(
                              ::std::mem::size_of::<u8>()
                          )
                      ) as (*mut u8)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                  };
            let mut new_id
                : *mut u16
                = if num_histograms != 0 {
                      BrotliAllocate(
                          m,
                          num_histograms.wrapping_mul(::std::mem::size_of::<u16>())
                      ) as (*mut u16)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
                  };
            let iters
                : usize
                = (if (*params).quality < 11i32 {
                       3i32
                   } else {
                       10i32
                   }) as (usize);
            let mut i : usize;
            if !(0i32 == 0) {
            } else {
                i = 0i32 as (usize);
                'loop7: loop {
                    if i < iters {
                        num_blocks = FindBlocksDistance(
                                         data,
                                         length,
                                         block_switch_cost,
                                         num_histograms,
                                         histograms as (*const HistogramDistance),
                                         insert_cost,
                                         cost,
                                         switch_signal,
                                         block_ids
                                     );
                        num_histograms = RemapBlockIdsDistance(
                                             block_ids,
                                             length,
                                             new_id,
                                             num_histograms
                                         );
                        BuildBlockHistogramsDistance(
                            data,
                            length,
                            block_ids as (*const u8),
                            num_histograms,
                            histograms
                        );
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop7;
                    } else {
                        break 'loop7;
                    }
                }
                BrotliFree(m,insert_cost as (*mut ::std::os::raw::c_void));
                insert_cost = 0i32 as (*mut ::std::os::raw::c_void) as (*mut f64);
                BrotliFree(m,cost as (*mut ::std::os::raw::c_void));
                cost = 0i32 as (*mut ::std::os::raw::c_void) as (*mut f64);
                BrotliFree(m,switch_signal as (*mut ::std::os::raw::c_void));
                switch_signal = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                BrotliFree(m,new_id as (*mut ::std::os::raw::c_void));
                new_id = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16);
                BrotliFree(m,histograms as (*mut ::std::os::raw::c_void));
                histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance);
                ClusterBlocksDistance(m,data,length,num_blocks,block_ids,split);
                if !(0i32 == 0) {
                } else {
                    BrotliFree(m,block_ids as (*mut ::std::os::raw::c_void));
                    block_ids = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliSplitBlock(
    mut m : *mut MemoryManager,
    mut cmds : *const Command,
    num_commands : usize,
    mut data : *const u8,
    pos : usize,
    mask : usize,
    mut params : *const BrotliEncoderParams,
    mut literal_split : *mut BlockSplit,
    mut insert_and_copy_split : *mut BlockSplit,
    mut dist_split : *mut BlockSplit
) {
    let mut literals_count : usize = CountLiterals(cmds,num_commands);
    let mut literals
        : *mut u8
        = if literals_count != 0 {
              BrotliAllocate(
                  m,
                  literals_count.wrapping_mul(::std::mem::size_of::<u8>())
              ) as (*mut u8)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
          };
    if !(0i32 == 0) {
    } else {
        CopyLiteralsToByteArray(cmds,num_commands,data,pos,mask,literals);
        SplitByteVectorLiteral(
            m,
            literals as (*const u8),
            literals_count,
            kSymbolsPerLiteralHistogram,
            kMaxLiteralHistograms,
            kLiteralStrideLength,
            kLiteralBlockSwitchCost,
            params,
            literal_split
        );
        if !(0i32 == 0) {
        } else {
            BrotliFree(m,literals as (*mut ::std::os::raw::c_void));
            literals = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
            let mut insert_and_copy_codes
                : *mut u16
                = if num_commands != 0 {
                      BrotliAllocate(
                          m,
                          num_commands.wrapping_mul(::std::mem::size_of::<u16>())
                      ) as (*mut u16)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
                  };
            let mut i : usize;
            if !(0i32 == 0) {
            } else {
                i = 0i32 as (usize);
                'loop4: loop {
                    if i < num_commands {
                        *insert_and_copy_codes.offset(i as (isize)) = (*cmds.offset(
                                                                            i as (isize)
                                                                        )).cmd_prefix_;
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop4;
                    } else {
                        break 'loop4;
                    }
                }
                SplitByteVectorCommand(
                    m,
                    insert_and_copy_codes as (*const u16),
                    num_commands,
                    kSymbolsPerCommandHistogram,
                    kMaxCommandHistograms,
                    kCommandStrideLength,
                    kCommandBlockSwitchCost,
                    params,
                    insert_and_copy_split
                );
                if !(0i32 == 0) {
                } else {
                    BrotliFree(
                        m,
                        insert_and_copy_codes as (*mut ::std::os::raw::c_void)
                    );
                    insert_and_copy_codes = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16);
                    let mut distance_prefixes
                        : *mut u16
                        = if num_commands != 0 {
                              BrotliAllocate(
                                  m,
                                  num_commands.wrapping_mul(::std::mem::size_of::<u16>())
                              ) as (*mut u16)
                          } else {
                              0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
                          };
                    let mut j : usize = 0i32 as (usize);
                    let mut i : usize;
                    if !(0i32 == 0) {
                    } else {
                        i = 0i32 as (usize);
                        'loop8: loop {
                            if i < num_commands {
                                let mut cmd
                                    : *const Command
                                    = &*cmds.offset(i as (isize)) as (*const Command);
                                if CommandCopyLen(
                                       cmd
                                   ) != 0 && ((*cmd).cmd_prefix_ as (i32) >= 128i32) {
                                    *distance_prefixes.offset(
                                         {
                                             let _old = j;
                                             j = j.wrapping_add(1 as (usize));
                                             _old
                                         } as (isize)
                                     ) = (*cmd).dist_prefix_;
                                }
                                i = i.wrapping_add(1 as (usize));
                                continue 'loop8;
                            } else {
                                break 'loop8;
                            }
                        }
                        SplitByteVectorDistance(
                            m,
                            distance_prefixes as (*const u16),
                            j,
                            kSymbolsPerDistanceHistogram,
                            kMaxCommandHistograms,
                            kCommandStrideLength,
                            kDistanceBlockSwitchCost,
                            params,
                            dist_split
                        );
                        if !(0i32 == 0) {
                        } else {
                            BrotliFree(m,distance_prefixes as (*mut ::std::os::raw::c_void));
                            distance_prefixes = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16);
                        }
                    }
                }
            }
        }
    }
}
