extern {
    fn BrotliAllocate(
        m : *mut MemoryManager, n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn BrotliBuildHistogramsWithContext(
        cmds : *const Command,
        num_commands : usize,
        literal_split : *const BlockSplit,
        insert_and_copy_split : *const BlockSplit,
        dist_split : *const BlockSplit,
        ringbuffer : *const u8,
        pos : usize,
        mask : usize,
        prev_byte : u8,
        prev_byte2 : u8,
        context_modes : *const ContextType,
        literal_histograms : *mut HistogramLiteral,
        insert_and_copy_histograms : *mut HistogramCommand,
        copy_dist_histograms : *mut HistogramDistance
    );
    fn BrotliClusterHistogramsDistance(
        m : *mut MemoryManager,
        in_ : *const HistogramDistance,
        in_size : usize,
        max_histograms : usize,
        out : *mut HistogramDistance,
        out_size : *mut usize,
        histogram_symbols : *mut u32
    );
    fn BrotliClusterHistogramsLiteral(
        m : *mut MemoryManager,
        in_ : *const HistogramLiteral,
        in_size : usize,
        max_histograms : usize,
        out : *mut HistogramLiteral,
        out_size : *mut usize,
        histogram_symbols : *mut u32
    );
    fn BrotliFree(
        m : *mut MemoryManager, p : *mut ::std::os::raw::c_void
    );
    fn BrotliOptimizeHuffmanCountsForRle(
        length : usize, counts : *mut u32, good_for_rle : *mut u8
    );
    fn BrotliSplitBlock(
        m : *mut MemoryManager,
        cmds : *const Command,
        num_commands : usize,
        data : *const u8,
        offset : usize,
        mask : usize,
        params : *const BrotliEncoderParams,
        literal_split : *mut BlockSplit,
        insert_and_copy_split : *mut BlockSplit,
        dist_split : *mut BlockSplit
    );
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

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
    pub opaque : *mut ::std::os::raw::c_void,
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
pub enum ContextType {
    CONTEXT_LSB6 = 0i32,
    CONTEXT_MSB6 = 1i32,
    CONTEXT_UTF8 = 2i32,
    CONTEXT_SIGNED = 3i32,
}

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

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_ : [u32; 256],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_ : [u32; 704],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramDistance {
    pub data_ : [u32; 520],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MetaBlockSplit {
    pub literal_split : BlockSplit,
    pub command_split : BlockSplit,
    pub distance_split : BlockSplit,
    pub literal_context_map : *mut u32,
    pub literal_context_map_size : usize,
    pub distance_context_map : *mut u32,
    pub distance_context_map_size : usize,
    pub literal_histograms : *mut HistogramLiteral,
    pub literal_histograms_size : usize,
    pub command_histograms : *mut HistogramCommand,
    pub command_histograms_size : usize,
    pub distance_histograms : *mut HistogramDistance,
    pub distance_histograms_size : usize,
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

#[no_mangle]
pub unsafe extern fn BrotliBuildMetaBlock(
    mut m : *mut MemoryManager,
    mut ringbuffer : *const u8,
    pos : usize,
    mask : usize,
    mut params : *const BrotliEncoderParams,
    mut prev_byte : u8,
    mut prev_byte2 : u8,
    mut cmds : *const Command,
    mut num_commands : usize,
    mut literal_context_mode : ContextType,
    mut mb : *mut MetaBlockSplit
) {
    static kMaxNumberOfHistograms : usize = 256i32 as (usize);
    let mut distance_histograms : *mut HistogramDistance;
    let mut literal_histograms : *mut HistogramLiteral;
    let mut literal_context_modes
        : *mut ContextType
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ContextType);
    let mut literal_histograms_size : usize;
    let mut distance_histograms_size : usize;
    let mut i : usize;
    let mut literal_context_multiplier : usize = 1i32 as (usize);
    BrotliSplitBlock(
        m,
        cmds,
        num_commands,
        ringbuffer,
        pos,
        mask,
        params,
        &mut (*mb).literal_split as (*mut BlockSplit),
        &mut (*mb).command_split as (*mut BlockSplit),
        &mut (*mb).distance_split as (*mut BlockSplit)
    );
    if !(0i32 == 0) {
    } else {
        if (*params).disable_literal_context_modeling == 0 {
            literal_context_multiplier = (1i32 << 6i32) as (usize);
            literal_context_modes = if (*mb).literal_split.num_types != 0 {
                                        BrotliAllocate(
                                            m,
                                            (*mb).literal_split.num_types.wrapping_mul(
                                                ::std::mem::size_of::<ContextType>()
                                            )
                                        ) as (*mut ContextType)
                                    } else {
                                        0i32 as (*mut ::std::os::raw::c_void) as (*mut ContextType)
                                    };
            if !(0i32 == 0) {
                return;
            } else {
                i = 0i32 as (usize);
                'loop4: loop {
                    if i < (*mb).literal_split.num_types {
                        *literal_context_modes.offset(i as (isize)) = literal_context_mode;
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop4;
                    } else {
                        break 'loop4;
                    }
                }
            }
        }
        literal_histograms_size = (*mb).literal_split.num_types.wrapping_mul(
                                      literal_context_multiplier
                                  );
        literal_histograms = if literal_histograms_size != 0 {
                                 BrotliAllocate(
                                     m,
                                     literal_histograms_size.wrapping_mul(
                                         ::std::mem::size_of::<HistogramLiteral>()
                                     )
                                 ) as (*mut HistogramLiteral)
                             } else {
                                 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral)
                             };
        if !(0i32 == 0) {
        } else {
            ClearHistogramsLiteral(literal_histograms,literal_histograms_size);
            distance_histograms_size = (*mb).distance_split.num_types << 2i32;
            distance_histograms = if distance_histograms_size != 0 {
                                      BrotliAllocate(
                                          m,
                                          distance_histograms_size.wrapping_mul(
                                              ::std::mem::size_of::<HistogramDistance>()
                                          )
                                      ) as (*mut HistogramDistance)
                                  } else {
                                      0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance)
                                  };
            if !(0i32 == 0) {
            } else {
                ClearHistogramsDistance(
                    distance_histograms,
                    distance_histograms_size
                );
                if (*mb).command_histograms == 0i32 as (*mut HistogramCommand) {
                    0i32;
                } else {
                    __assert_fail(
                        (*b"mb->command_histograms == 0\0").as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        (*b"BrotliBuildMetaBlock\0").as_ptr()
                    );
                }
                (*mb).command_histograms_size = (*mb).command_split.num_types;
                (*mb).command_histograms = if (*mb).command_histograms_size != 0 {
                                               BrotliAllocate(
                                                   m,
                                                   (*mb).command_histograms_size.wrapping_mul(
                                                       ::std::mem::size_of::<HistogramCommand>()
                                                   )
                                               ) as (*mut HistogramCommand)
                                           } else {
                                               0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand)
                                           };
                if !(0i32 == 0) {
                } else {
                    ClearHistogramsCommand(
                        (*mb).command_histograms,
                        (*mb).command_histograms_size
                    );
                    BrotliBuildHistogramsWithContext(
                        cmds,
                        num_commands,
                        &mut (*mb).literal_split as (*mut BlockSplit) as (*const BlockSplit),
                        &mut (*mb).command_split as (*mut BlockSplit) as (*const BlockSplit),
                        &mut (*mb).distance_split as (*mut BlockSplit) as (*const BlockSplit),
                        ringbuffer,
                        pos,
                        mask,
                        prev_byte,
                        prev_byte2,
                        literal_context_modes as (*const ContextType),
                        literal_histograms,
                        (*mb).command_histograms,
                        distance_histograms
                    );
                    BrotliFree(
                        m,
                        literal_context_modes as (*mut ::std::os::raw::c_void)
                    );
                    literal_context_modes = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ContextType);
                    if (*mb).literal_context_map == 0i32 as (*mut u32) {
                        0i32;
                    } else {
                        __assert_fail(
                            (*b"mb->literal_context_map == 0\0").as_ptr(),
                            file!().as_ptr(),
                            line!(),
                            (*b"BrotliBuildMetaBlock\0").as_ptr()
                        );
                    }
                    (*mb).literal_context_map_size = (*mb).literal_split.num_types << 6i32;
                    (*mb).literal_context_map = if (*mb).literal_context_map_size != 0 {
                                                    BrotliAllocate(
                                                        m,
                                                        (*mb).literal_context_map_size.wrapping_mul(
                                                            ::std::mem::size_of::<u32>()
                                                        )
                                                    ) as (*mut u32)
                                                } else {
                                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                                };
                    if !(0i32 == 0) {
                    } else {
                        if (*mb).literal_histograms == 0i32 as (*mut HistogramLiteral) {
                            0i32;
                        } else {
                            __assert_fail(
                                (*b"mb->literal_histograms == 0\0").as_ptr(),
                                file!().as_ptr(),
                                line!(),
                                (*b"BrotliBuildMetaBlock\0").as_ptr()
                            );
                        }
                        (*mb).literal_histograms_size = (*mb).literal_context_map_size;
                        (*mb).literal_histograms = if (*mb).literal_histograms_size != 0 {
                                                       BrotliAllocate(
                                                           m,
                                                           (*mb).literal_histograms_size.wrapping_mul(
                                                               ::std::mem::size_of::<HistogramLiteral>(
                                                               )
                                                           )
                                                       ) as (*mut HistogramLiteral)
                                                   } else {
                                                       0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral)
                                                   };
                        if !(0i32 == 0) {
                        } else {
                            BrotliClusterHistogramsLiteral(
                                m,
                                literal_histograms as (*const HistogramLiteral),
                                literal_histograms_size,
                                kMaxNumberOfHistograms,
                                (*mb).literal_histograms,
                                &mut (*mb).literal_histograms_size as (*mut usize),
                                (*mb).literal_context_map
                            );
                            if !(0i32 == 0) {
                            } else {
                                BrotliFree(m,literal_histograms as (*mut ::std::os::raw::c_void));
                                literal_histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral);
                                if (*params).disable_literal_context_modeling != 0 {
                                    i = (*mb).literal_split.num_types;
                                    'loop13: loop {
                                        if i != 0i32 as (usize) {
                                            let mut j : usize = 0i32 as (usize);
                                            i = i.wrapping_sub(1 as (usize));
                                            'loop22: loop {
                                                if j < (1i32 << 6i32) as (usize) {
                                                    *(*mb).literal_context_map.offset(
                                                         (i << 6i32).wrapping_add(j) as (isize)
                                                     ) = *(*mb).literal_context_map.offset(
                                                              i as (isize)
                                                          );
                                                    j = j.wrapping_add(1 as (usize));
                                                    continue 'loop22;
                                                } else {
                                                    continue 'loop13;
                                                }
                                            }
                                        } else {
                                            break 'loop13;
                                        }
                                    }
                                }
                                if (*mb).distance_context_map == 0i32 as (*mut u32) {
                                    0i32;
                                } else {
                                    __assert_fail(
                                        (*b"mb->distance_context_map == 0\0").as_ptr(),
                                        file!().as_ptr(),
                                        line!(),
                                        (*b"BrotliBuildMetaBlock\0").as_ptr()
                                    );
                                }
                                (*mb).distance_context_map_size = (*mb).distance_split.num_types << 2i32;
                                (*mb).distance_context_map = if (*mb).distance_context_map_size != 0 {
                                                                 BrotliAllocate(
                                                                     m,
                                                                     (*mb).distance_context_map_size.wrapping_mul(
                                                                         ::std::mem::size_of::<u32>(
                                                                         )
                                                                     )
                                                                 ) as (*mut u32)
                                                             } else {
                                                                 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                                             };
                                if !(0i32 == 0) {
                                } else {
                                    if (*mb).distance_histograms == 0i32 as (*mut HistogramDistance) {
                                        0i32;
                                    } else {
                                        __assert_fail(
                                            (*b"mb->distance_histograms == 0\0").as_ptr(),
                                            file!().as_ptr(),
                                            line!(),
                                            (*b"BrotliBuildMetaBlock\0").as_ptr()
                                        );
                                    }
                                    (*mb).distance_histograms_size = (*mb).distance_context_map_size;
                                    (*mb).distance_histograms = if (*mb).distance_histograms_size != 0 {
                                                                    BrotliAllocate(
                                                                        m,
                                                                        (*mb).distance_histograms_size.wrapping_mul(
                                                                            ::std::mem::size_of::<HistogramDistance>(
                                                                            )
                                                                        )
                                                                    ) as (*mut HistogramDistance)
                                                                } else {
                                                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance)
                                                                };
                                    if !(0i32 == 0) {
                                    } else {
                                        BrotliClusterHistogramsDistance(
                                            m,
                                            distance_histograms as (*const HistogramDistance),
                                            (*mb).distance_context_map_size,
                                            kMaxNumberOfHistograms,
                                            (*mb).distance_histograms,
                                            &mut (*mb).distance_histograms_size as (*mut usize),
                                            (*mb).distance_context_map
                                        );
                                        if !(0i32 == 0) {
                                        } else {
                                            BrotliFree(
                                                m,
                                                distance_histograms as (*mut ::std::os::raw::c_void)
                                            );
                                            distance_histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockSplitterLiteral {
    pub alphabet_size_ : usize,
    pub min_block_size_ : usize,
    pub split_threshold_ : f64,
    pub num_blocks_ : usize,
    pub split_ : *mut BlockSplit,
    pub histograms_ : *mut HistogramLiteral,
    pub histograms_size_ : *mut usize,
    pub target_block_size_ : usize,
    pub block_size_ : usize,
    pub curr_histogram_ix_ : usize,
    pub last_histogram_ix_ : [usize; 2],
    pub last_entropy_ : [f64; 2],
    pub merge_last_count_ : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContextBlockSplitter {
    pub alphabet_size_ : usize,
    pub num_contexts_ : usize,
    pub max_block_types_ : usize,
    pub min_block_size_ : usize,
    pub split_threshold_ : f64,
    pub num_blocks_ : usize,
    pub split_ : *mut BlockSplit,
    pub histograms_ : *mut HistogramLiteral,
    pub histograms_size_ : *mut usize,
    pub target_block_size_ : usize,
    pub block_size_ : usize,
    pub curr_histogram_ix_ : usize,
    pub last_histogram_ix_ : [usize; 2],
    pub last_entropy_ : [f64; 6],
    pub merge_last_count_ : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LitBlocks {
    pub plain : BlockSplitterLiteral,
    pub ctx : ContextBlockSplitter,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockSplitterCommand {
    pub alphabet_size_ : usize,
    pub min_block_size_ : usize,
    pub split_threshold_ : f64,
    pub num_blocks_ : usize,
    pub split_ : *mut BlockSplit,
    pub histograms_ : *mut HistogramCommand,
    pub histograms_size_ : *mut usize,
    pub target_block_size_ : usize,
    pub block_size_ : usize,
    pub curr_histogram_ix_ : usize,
    pub last_histogram_ix_ : [usize; 2],
    pub last_entropy_ : [f64; 2],
    pub merge_last_count_ : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockSplitterDistance {
    pub alphabet_size_ : usize,
    pub min_block_size_ : usize,
    pub split_threshold_ : f64,
    pub num_blocks_ : usize,
    pub split_ : *mut BlockSplit,
    pub histograms_ : *mut HistogramDistance,
    pub histograms_size_ : *mut usize,
    pub target_block_size_ : usize,
    pub block_size_ : usize,
    pub curr_histogram_ix_ : usize,
    pub last_histogram_ix_ : [usize; 2],
    pub last_entropy_ : [f64; 2],
    pub merge_last_count_ : usize,
}

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
}

unsafe extern fn InitContextBlockSplitter(
    mut m : *mut MemoryManager,
    mut self : *mut ContextBlockSplitter,
    mut alphabet_size : usize,
    mut num_contexts : usize,
    mut min_block_size : usize,
    mut split_threshold : f64,
    mut num_symbols : usize,
    mut split : *mut BlockSplit,
    mut histograms : *mut *mut HistogramLiteral,
    mut histograms_size : *mut usize
) {
    let mut max_num_blocks
        : usize
        = num_symbols.wrapping_div(min_block_size).wrapping_add(
              1i32 as (usize)
          );
    let mut max_num_types : usize;
    if num_contexts <= 3i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            (*b"num_contexts <= BROTLI_MAX_STATIC_CONTEXTS\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"InitContextBlockSplitter\0").as_ptr()
        );
    }
    (*self).alphabet_size_ = alphabet_size;
    (*self).num_contexts_ = num_contexts;
    (*self).max_block_types_ = (256i32 as (usize)).wrapping_div(
                                   num_contexts
                               );
    (*self).min_block_size_ = min_block_size;
    (*self).split_threshold_ = split_threshold;
    (*self).num_blocks_ = 0i32 as (usize);
    (*self).split_ = split;
    (*self).histograms_size_ = histograms_size;
    (*self).target_block_size_ = min_block_size;
    (*self).block_size_ = 0i32 as (usize);
    (*self).curr_histogram_ix_ = 0i32 as (usize);
    (*self).merge_last_count_ = 0i32 as (usize);
    max_num_types = brotli_min_size_t(
                        max_num_blocks,
                        (*self).max_block_types_.wrapping_add(1i32 as (usize))
                    );
    if (*split).types_alloc_size < max_num_blocks {
        let mut _new_size
            : usize
            = if (*split).types_alloc_size == 0i32 as (usize) {
                  max_num_blocks
              } else {
                  (*split).types_alloc_size
              };
        let mut new_array : *mut u8;
        'loop2: loop {
            if _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
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
    if (*split).lengths_alloc_size < max_num_blocks {
        let mut _new_size
            : usize
            = if (*split).lengths_alloc_size == 0i32 as (usize) {
                  max_num_blocks
              } else {
                  (*split).lengths_alloc_size
              };
        let mut new_array : *mut u32;
        'loop8: loop {
            if _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
                continue 'loop8;
            } else {
                break 'loop8;
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
        (*split).num_blocks = max_num_blocks;
        if !(0i32 == 0) {
        } else {
            if *histograms == 0i32 as (*mut HistogramLiteral) {
                0i32;
            } else {
                __assert_fail(
                    (*b"*histograms == 0\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"InitContextBlockSplitter\0").as_ptr()
                );
            }
            *histograms_size = max_num_types.wrapping_mul(num_contexts);
            *histograms = if *histograms_size != 0 {
                              BrotliAllocate(
                                  m,
                                  (*histograms_size).wrapping_mul(
                                      ::std::mem::size_of::<HistogramLiteral>()
                                  )
                              ) as (*mut HistogramLiteral)
                          } else {
                              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral)
                          };
            (*self).histograms_ = *histograms;
            if !(0i32 == 0) {
            } else {
                ClearHistogramsLiteral(
                    &mut *(*self).histograms_.offset(
                              0i32 as (isize)
                          ) as (*mut HistogramLiteral),
                    num_contexts
                );
                (*self).last_histogram_ix_[0i32 as (usize)] = {
                                                                  let _rhs = 0i32;
                                                                  let _lhs
                                                                      = &mut (*self).last_histogram_ix_[
                                                                                 1i32 as (usize)
                                                                             ];
                                                                  *_lhs = _rhs as (usize);
                                                                  *_lhs
                                                              };
            }
        }
    }
}

unsafe extern fn InitBlockSplitterLiteral(
    mut m : *mut MemoryManager,
    mut self : *mut BlockSplitterLiteral,
    mut alphabet_size : usize,
    mut min_block_size : usize,
    mut split_threshold : f64,
    mut num_symbols : usize,
    mut split : *mut BlockSplit,
    mut histograms : *mut *mut HistogramLiteral,
    mut histograms_size : *mut usize
) {
    let mut max_num_blocks
        : usize
        = num_symbols.wrapping_div(min_block_size).wrapping_add(
              1i32 as (usize)
          );
    let mut max_num_types
        : usize
        = brotli_min_size_t(max_num_blocks,(256i32 + 1i32) as (usize));
    (*self).alphabet_size_ = alphabet_size;
    (*self).min_block_size_ = min_block_size;
    (*self).split_threshold_ = split_threshold;
    (*self).num_blocks_ = 0i32 as (usize);
    (*self).split_ = split;
    (*self).histograms_size_ = histograms_size;
    (*self).target_block_size_ = min_block_size;
    (*self).block_size_ = 0i32 as (usize);
    (*self).curr_histogram_ix_ = 0i32 as (usize);
    (*self).merge_last_count_ = 0i32 as (usize);
    if (*split).types_alloc_size < max_num_blocks {
        let mut _new_size
            : usize
            = if (*split).types_alloc_size == 0i32 as (usize) {
                  max_num_blocks
              } else {
                  (*split).types_alloc_size
              };
        let mut new_array : *mut u8;
        'loop2: loop {
            if _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
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
    if (*split).lengths_alloc_size < max_num_blocks {
        let mut _new_size
            : usize
            = if (*split).lengths_alloc_size == 0i32 as (usize) {
                  max_num_blocks
              } else {
                  (*split).lengths_alloc_size
              };
        let mut new_array : *mut u32;
        'loop8: loop {
            if _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
                continue 'loop8;
            } else {
                break 'loop8;
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
        (*(*self).split_).num_blocks = max_num_blocks;
        if *histograms == 0i32 as (*mut HistogramLiteral) {
            0i32;
        } else {
            __assert_fail(
                (*b"*histograms == 0\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"InitBlockSplitterLiteral\0").as_ptr()
            );
        }
        *histograms_size = max_num_types;
        *histograms = if *histograms_size != 0 {
                          BrotliAllocate(
                              m,
                              (*histograms_size).wrapping_mul(
                                  ::std::mem::size_of::<HistogramLiteral>()
                              )
                          ) as (*mut HistogramLiteral)
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral)
                      };
        (*self).histograms_ = *histograms;
        if !(0i32 == 0) {
        } else {
            HistogramClearLiteral(
                &mut *(*self).histograms_.offset(
                          0i32 as (isize)
                      ) as (*mut HistogramLiteral)
            );
            (*self).last_histogram_ix_[0i32 as (usize)] = {
                                                              let _rhs = 0i32;
                                                              let _lhs
                                                                  = &mut (*self).last_histogram_ix_[
                                                                             1i32 as (usize)
                                                                         ];
                                                              *_lhs = _rhs as (usize);
                                                              *_lhs
                                                          };
        }
    }
}

unsafe extern fn InitBlockSplitterCommand(
    mut m : *mut MemoryManager,
    mut self : *mut BlockSplitterCommand,
    mut alphabet_size : usize,
    mut min_block_size : usize,
    mut split_threshold : f64,
    mut num_symbols : usize,
    mut split : *mut BlockSplit,
    mut histograms : *mut *mut HistogramCommand,
    mut histograms_size : *mut usize
) {
    let mut max_num_blocks
        : usize
        = num_symbols.wrapping_div(min_block_size).wrapping_add(
              1i32 as (usize)
          );
    let mut max_num_types
        : usize
        = brotli_min_size_t(max_num_blocks,(256i32 + 1i32) as (usize));
    (*self).alphabet_size_ = alphabet_size;
    (*self).min_block_size_ = min_block_size;
    (*self).split_threshold_ = split_threshold;
    (*self).num_blocks_ = 0i32 as (usize);
    (*self).split_ = split;
    (*self).histograms_size_ = histograms_size;
    (*self).target_block_size_ = min_block_size;
    (*self).block_size_ = 0i32 as (usize);
    (*self).curr_histogram_ix_ = 0i32 as (usize);
    (*self).merge_last_count_ = 0i32 as (usize);
    if (*split).types_alloc_size < max_num_blocks {
        let mut _new_size
            : usize
            = if (*split).types_alloc_size == 0i32 as (usize) {
                  max_num_blocks
              } else {
                  (*split).types_alloc_size
              };
        let mut new_array : *mut u8;
        'loop2: loop {
            if _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
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
    if (*split).lengths_alloc_size < max_num_blocks {
        let mut _new_size
            : usize
            = if (*split).lengths_alloc_size == 0i32 as (usize) {
                  max_num_blocks
              } else {
                  (*split).lengths_alloc_size
              };
        let mut new_array : *mut u32;
        'loop8: loop {
            if _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
                continue 'loop8;
            } else {
                break 'loop8;
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
        (*(*self).split_).num_blocks = max_num_blocks;
        if *histograms == 0i32 as (*mut HistogramCommand) {
            0i32;
        } else {
            __assert_fail(
                (*b"*histograms == 0\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"InitBlockSplitterCommand\0").as_ptr()
            );
        }
        *histograms_size = max_num_types;
        *histograms = if *histograms_size != 0 {
                          BrotliAllocate(
                              m,
                              (*histograms_size).wrapping_mul(
                                  ::std::mem::size_of::<HistogramCommand>()
                              )
                          ) as (*mut HistogramCommand)
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand)
                      };
        (*self).histograms_ = *histograms;
        if !(0i32 == 0) {
        } else {
            HistogramClearCommand(
                &mut *(*self).histograms_.offset(
                          0i32 as (isize)
                      ) as (*mut HistogramCommand)
            );
            (*self).last_histogram_ix_[0i32 as (usize)] = {
                                                              let _rhs = 0i32;
                                                              let _lhs
                                                                  = &mut (*self).last_histogram_ix_[
                                                                             1i32 as (usize)
                                                                         ];
                                                              *_lhs = _rhs as (usize);
                                                              *_lhs
                                                          };
        }
    }
}

unsafe extern fn InitBlockSplitterDistance(
    mut m : *mut MemoryManager,
    mut self : *mut BlockSplitterDistance,
    mut alphabet_size : usize,
    mut min_block_size : usize,
    mut split_threshold : f64,
    mut num_symbols : usize,
    mut split : *mut BlockSplit,
    mut histograms : *mut *mut HistogramDistance,
    mut histograms_size : *mut usize
) {
    let mut max_num_blocks
        : usize
        = num_symbols.wrapping_div(min_block_size).wrapping_add(
              1i32 as (usize)
          );
    let mut max_num_types
        : usize
        = brotli_min_size_t(max_num_blocks,(256i32 + 1i32) as (usize));
    (*self).alphabet_size_ = alphabet_size;
    (*self).min_block_size_ = min_block_size;
    (*self).split_threshold_ = split_threshold;
    (*self).num_blocks_ = 0i32 as (usize);
    (*self).split_ = split;
    (*self).histograms_size_ = histograms_size;
    (*self).target_block_size_ = min_block_size;
    (*self).block_size_ = 0i32 as (usize);
    (*self).curr_histogram_ix_ = 0i32 as (usize);
    (*self).merge_last_count_ = 0i32 as (usize);
    if (*split).types_alloc_size < max_num_blocks {
        let mut _new_size
            : usize
            = if (*split).types_alloc_size == 0i32 as (usize) {
                  max_num_blocks
              } else {
                  (*split).types_alloc_size
              };
        let mut new_array : *mut u8;
        'loop2: loop {
            if _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
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
    if (*split).lengths_alloc_size < max_num_blocks {
        let mut _new_size
            : usize
            = if (*split).lengths_alloc_size == 0i32 as (usize) {
                  max_num_blocks
              } else {
                  (*split).lengths_alloc_size
              };
        let mut new_array : *mut u32;
        'loop8: loop {
            if _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
                continue 'loop8;
            } else {
                break 'loop8;
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
        (*(*self).split_).num_blocks = max_num_blocks;
        if *histograms == 0i32 as (*mut HistogramDistance) {
            0i32;
        } else {
            __assert_fail(
                (*b"*histograms == 0\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"InitBlockSplitterDistance\0").as_ptr()
            );
        }
        *histograms_size = max_num_types;
        *histograms = if *histograms_size != 0 {
                          BrotliAllocate(
                              m,
                              (*histograms_size).wrapping_mul(
                                  ::std::mem::size_of::<HistogramDistance>()
                              )
                          ) as (*mut HistogramDistance)
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance)
                      };
        (*self).histograms_ = *histograms;
        if !(0i32 == 0) {
        } else {
            HistogramClearDistance(
                &mut *(*self).histograms_.offset(
                          0i32 as (isize)
                      ) as (*mut HistogramDistance)
            );
            (*self).last_histogram_ix_[0i32 as (usize)] = {
                                                              let _rhs = 0i32;
                                                              let _lhs
                                                                  = &mut (*self).last_histogram_ix_[
                                                                             1i32 as (usize)
                                                                         ];
                                                              *_lhs = _rhs as (usize);
                                                              *_lhs
                                                          };
        }
    }
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

unsafe extern fn brotli_max_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a > b { a } else { b }
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

unsafe extern fn ShannonEntropy(
    mut population : *const u32,
    mut size : usize,
    mut total : *mut usize
) -> f64 {
    let mut sum : usize = 0i32 as (usize);
    let mut retval : f64 = 0i32 as (f64);
    let mut population_end
        : *const u32
        = population.offset(size as (isize));
    let mut p : usize;
    let mut odd_number_of_elements_left : i32 = 0i32;
    if size & 1i32 as (usize) != 0 {
        odd_number_of_elements_left = 1i32;
    }
    'loop2: loop {
        if population < population_end {
            if odd_number_of_elements_left == 0 {
                p = *{
                         let _old = population;
                         population = population.offset(1 as (isize));
                         _old
                     } as (usize);
                sum = sum.wrapping_add(p);
                retval = retval - p as (f64) * FastLog2(p);
            }
            odd_number_of_elements_left = 0i32;
            p = *{
                     let _old = population;
                     population = population.offset(1 as (isize));
                     _old
                 } as (usize);
            sum = sum.wrapping_add(p);
            retval = retval - p as (f64) * FastLog2(p);
            continue 'loop2;
        } else {
            break 'loop2;
        }
    }
    if sum != 0 {
        retval = retval + sum as (f64) * FastLog2(sum);
    }
    *total = sum;
    retval
}

unsafe extern fn BitsEntropy(
    mut population : *const u32, mut size : usize
) -> f64 {
    let mut sum : usize;
    let mut retval
        : f64
        = ShannonEntropy(population,size,&mut sum as (*mut usize));
    if retval < sum as (f64) {
        retval = sum as (f64);
    }
    retval
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

unsafe extern fn BlockSplitterFinishBlockCommand(
    mut self : *mut BlockSplitterCommand, mut is_final : i32
) {
    let mut split : *mut BlockSplit = (*self).split_;
    let mut last_entropy
        : *mut f64
        = (*self).last_entropy_.as_mut_ptr();
    let mut histograms : *mut HistogramCommand = (*self).histograms_;
    (*self).block_size_ = brotli_max_size_t(
                              (*self).block_size_,
                              (*self).min_block_size_
                          );
    if (*self).num_blocks_ == 0i32 as (usize) {
        *(*split).lengths.offset(
             0i32 as (isize)
         ) = (*self).block_size_ as (u32);
        *(*split).types.offset(0i32 as (isize)) = 0i32 as (u8);
        *last_entropy.offset(0i32 as (isize)) = BitsEntropy(
                                                    (*histograms.offset(
                                                          0i32 as (isize)
                                                      )).data_.as_mut_ptr(
                                                    ) as (*const u32),
                                                    (*self).alphabet_size_
                                                );
        *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                     0i32 as (isize)
                                                 );
        (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                  1 as (usize)
                              );
        (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
        (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                         1 as (usize)
                                     );
        if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
            HistogramClearCommand(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramCommand)
            );
        }
        (*self).block_size_ = 0i32 as (usize);
    } else if (*self).block_size_ > 0i32 as (usize) {
        let mut entropy
            : f64
            = BitsEntropy(
                  (*histograms.offset(
                        (*self).curr_histogram_ix_ as (isize)
                    )).data_.as_mut_ptr(
                  ) as (*const u32),
                  (*self).alphabet_size_
              );
        let mut combined_histo : [HistogramCommand; 2];
        let mut combined_entropy : [f64; 2];
        let mut diff : [f64; 2];
        let mut j : usize;
        j = 0i32 as (usize);
        'loop3: loop {
            if j < 2i32 as (usize) {
                let mut last_histogram_ix : usize = (*self).last_histogram_ix_[j];
                combined_histo[j] = *histograms.offset(
                                         (*self).curr_histogram_ix_ as (isize)
                                     );
                HistogramAddHistogramCommand(
                    &mut combined_histo[j] as (*mut HistogramCommand),
                    &mut *histograms.offset(
                              last_histogram_ix as (isize)
                          ) as (*mut HistogramCommand) as (*const HistogramCommand)
                );
                combined_entropy[j] = BitsEntropy(
                                          &mut combined_histo[j].data_[
                                                   0i32 as (usize)
                                               ] as (*mut u32) as (*const u32),
                                          (*self).alphabet_size_
                                      );
                diff[j] = combined_entropy[j] - entropy - *last_entropy.offset(
                                                               j as (isize)
                                                           );
                j = j.wrapping_add(1 as (usize));
                continue 'loop3;
            } else {
                break 'loop3;
            }
        }
        if (*split).num_types < 256i32 as (usize) && (diff[
                                                          0i32 as (usize)
                                                      ] > (*self).split_threshold_) && (diff[
                                                                                            1i32 as (usize)
                                                                                        ] > (*self).split_threshold_) {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*split).num_types as (u8);
            (*self).last_histogram_ix_[
                1i32 as (usize)
            ] = (*self).last_histogram_ix_[0i32 as (usize)];
            (*self).last_histogram_ix_[
                0i32 as (usize)
            ] = (*split).num_types as (u8) as (usize);
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = entropy;
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
            (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                             1 as (usize)
                                         );
            if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
                HistogramClearCommand(
                    &mut *histograms.offset(
                              (*self).curr_histogram_ix_ as (isize)
                          ) as (*mut HistogramCommand)
                );
            }
            (*self).block_size_ = 0i32 as (usize);
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else if diff[1i32 as (usize)] < diff[0i32 as (usize)] - 20.0f64 {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = *(*split).types.offset(
                      (*self).num_blocks_.wrapping_sub(2i32 as (usize)) as (isize)
                  );
            let mut __brotli_swap_tmp
                : usize
                = (*self).last_histogram_ix_[0i32 as (usize)];
            (*self).last_histogram_ix_[
                0i32 as (usize)
            ] = (*self).last_histogram_ix_[1i32 as (usize)];
            (*self).last_histogram_ix_[1i32 as (usize)] = __brotli_swap_tmp;
            *histograms.offset(
                 (*self).last_histogram_ix_[0i32 as (usize)] as (isize)
             ) = combined_histo[1i32 as (usize)];
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = combined_entropy[
                                                        1i32 as (usize)
                                                    ];
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearCommand(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramCommand)
            );
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else {
            {
                let _rhs = (*self).block_size_ as (u32);
                let _lhs
                    = &mut *(*split).lengths.offset(
                                (*self).num_blocks_.wrapping_sub(1i32 as (usize)) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            *histograms.offset(
                 (*self).last_histogram_ix_[0i32 as (usize)] as (isize)
             ) = combined_histo[0i32 as (usize)];
            *last_entropy.offset(0i32 as (isize)) = combined_entropy[
                                                        0i32 as (usize)
                                                    ];
            if (*split).num_types == 1i32 as (usize) {
                *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                             0i32 as (isize)
                                                         );
            }
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearCommand(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramCommand)
            );
            if {
                   (*self).merge_last_count_ = (*self).merge_last_count_.wrapping_add(
                                                   1 as (usize)
                                               );
                   (*self).merge_last_count_
               } > 1i32 as (usize) {
                (*self).target_block_size_ = (*self).target_block_size_.wrapping_add(
                                                 (*self).min_block_size_
                                             );
            }
        }
    }
    if is_final != 0 {
        *(*self).histograms_size_ = (*split).num_types;
        (*split).num_blocks = (*self).num_blocks_;
    }
}

unsafe extern fn BlockSplitterAddSymbolCommand(
    mut self : *mut BlockSplitterCommand, mut symbol : usize
) {
    HistogramAddCommand(
        &mut *(*self).histograms_.offset(
                  (*self).curr_histogram_ix_ as (isize)
              ) as (*mut HistogramCommand),
        symbol
    );
    (*self).block_size_ = (*self).block_size_.wrapping_add(
                              1 as (usize)
                          );
    if (*self).block_size_ == (*self).target_block_size_ {
        BlockSplitterFinishBlockCommand(self,0i32);
    }
}

unsafe extern fn Context(
    mut p1 : u8, mut p2 : u8, mut mode : ContextType
) -> u8 {
    if mode as (i32) == ContextType::CONTEXT_SIGNED as (i32) {
        ((kSigned3BitContextLookup[
              p1 as (usize)
          ] as (i32) << 3i32) + kSigned3BitContextLookup[
                                    p2 as (usize)
                                ] as (i32)) as (u8)
    } else if mode as (i32) == ContextType::CONTEXT_UTF8 as (i32) {
        (kUTF8ContextLookup[p1 as (usize)] as (i32) | kUTF8ContextLookup[
                                                          (p2 as (i32) + 256i32) as (usize)
                                                      ] as (i32)) as (u8)
    } else if mode as (i32) == ContextType::CONTEXT_MSB6 as (i32) {
        (p1 as (i32) >> 2i32) as (u8)
    } else if mode as (i32) == ContextType::CONTEXT_LSB6 as (i32) {
        (p1 as (i32) & 0x3fi32) as (u8)
    } else {
        0i32 as (u8)
    }
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

unsafe extern fn ContextBlockSplitterFinishBlock(
    mut self : *mut ContextBlockSplitter, mut is_final : i32
) {
    let mut split : *mut BlockSplit = (*self).split_;
    let num_contexts : usize = (*self).num_contexts_;
    let mut last_entropy
        : *mut f64
        = (*self).last_entropy_.as_mut_ptr();
    let mut histograms : *mut HistogramLiteral = (*self).histograms_;
    if (*self).block_size_ < (*self).min_block_size_ {
        (*self).block_size_ = (*self).min_block_size_;
    }
    if (*self).num_blocks_ == 0i32 as (usize) {
        let mut i : usize;
        *(*split).lengths.offset(
             0i32 as (isize)
         ) = (*self).block_size_ as (u32);
        *(*split).types.offset(0i32 as (isize)) = 0i32 as (u8);
        i = 0i32 as (usize);
        'loop34: loop {
            if i < num_contexts {
                *last_entropy.offset(i as (isize)) = BitsEntropy(
                                                         (*histograms.offset(
                                                               i as (isize)
                                                           )).data_.as_mut_ptr(
                                                         ) as (*const u32),
                                                         (*self).alphabet_size_
                                                     );
                *last_entropy.offset(
                     num_contexts.wrapping_add(i) as (isize)
                 ) = *last_entropy.offset(i as (isize));
                i = i.wrapping_add(1 as (usize));
                continue 'loop34;
            } else {
                break 'loop34;
            }
        }
        (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                  1 as (usize)
                              );
        (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
        (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                         num_contexts
                                     );
        if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
            ClearHistogramsLiteral(
                &mut *(*self).histograms_.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramLiteral),
                (*self).num_contexts_
            );
        }
        (*self).block_size_ = 0i32 as (usize);
    } else if (*self).block_size_ > 0i32 as (usize) {
        let mut entropy : [f64; 3];
        let mut combined_histo : [HistogramLiteral; 6];
        let mut combined_entropy : [f64; 6];
        let mut diff : [f64; 2] = [ 0.0f64, 0.0f64 ];
        let mut i : usize;
        i = 0i32 as (usize);
        'loop5: loop {
            if i < num_contexts {
                let mut curr_histo_ix
                    : usize
                    = (*self).curr_histogram_ix_.wrapping_add(i);
                let mut j : usize;
                entropy[i] = BitsEntropy(
                                 (*histograms.offset(curr_histo_ix as (isize))).data_.as_mut_ptr(
                                 ) as (*const u32),
                                 (*self).alphabet_size_
                             );
                j = 0i32 as (usize);
                'loop29: loop {
                    if j < 2i32 as (usize) {
                        let mut jx : usize = j.wrapping_mul(num_contexts).wrapping_add(i);
                        let mut last_histogram_ix
                            : usize
                            = (*self).last_histogram_ix_[j].wrapping_add(i);
                        combined_histo[jx] = *histograms.offset(curr_histo_ix as (isize));
                        HistogramAddHistogramLiteral(
                            &mut combined_histo[jx] as (*mut HistogramLiteral),
                            &mut *histograms.offset(
                                      last_histogram_ix as (isize)
                                  ) as (*mut HistogramLiteral) as (*const HistogramLiteral)
                        );
                        combined_entropy[jx] = BitsEntropy(
                                                   &mut combined_histo[jx].data_[
                                                            0i32 as (usize)
                                                        ] as (*mut u32) as (*const u32),
                                                   (*self).alphabet_size_
                                               );
                        {
                            let _rhs
                                = combined_entropy[jx] - entropy[i] - *last_entropy.offset(
                                                                           jx as (isize)
                                                                       );
                            let _lhs = &mut diff[j];
                            *_lhs = *_lhs + _rhs;
                        }
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop29;
                    } else {
                        break 'loop29;
                    }
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop5;
            } else {
                break 'loop5;
            }
        }
        if (*split).num_types < (*self).max_block_types_ && (diff[
                                                                 0i32 as (usize)
                                                             ] > (*self).split_threshold_) && (diff[
                                                                                                   1i32 as (usize)
                                                                                               ] > (*self).split_threshold_) {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*split).num_types as (u8);
            (*self).last_histogram_ix_[
                1i32 as (usize)
            ] = (*self).last_histogram_ix_[0i32 as (usize)];
            (*self).last_histogram_ix_[
                0i32 as (usize)
            ] = (*split).num_types.wrapping_mul(num_contexts);
            i = 0i32 as (usize);
            'loop22: loop {
                if i < num_contexts {
                    *last_entropy.offset(
                         num_contexts.wrapping_add(i) as (isize)
                     ) = *last_entropy.offset(i as (isize));
                    *last_entropy.offset(i as (isize)) = entropy[i];
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop22;
                } else {
                    break 'loop22;
                }
            }
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
            (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                             num_contexts
                                         );
            if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
                ClearHistogramsLiteral(
                    &mut *(*self).histograms_.offset(
                              (*self).curr_histogram_ix_ as (isize)
                          ) as (*mut HistogramLiteral),
                    (*self).num_contexts_
                );
            }
            (*self).block_size_ = 0i32 as (usize);
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else if diff[1i32 as (usize)] < diff[0i32 as (usize)] - 20.0f64 {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = *(*split).types.offset(
                      (*self).num_blocks_.wrapping_sub(2i32 as (usize)) as (isize)
                  );
            let mut __brotli_swap_tmp
                : usize
                = (*self).last_histogram_ix_[0i32 as (usize)];
            (*self).last_histogram_ix_[
                0i32 as (usize)
            ] = (*self).last_histogram_ix_[1i32 as (usize)];
            (*self).last_histogram_ix_[1i32 as (usize)] = __brotli_swap_tmp;
            i = 0i32 as (usize);
            'loop17: loop {
                if i < num_contexts {
                    *histograms.offset(
                         (*self).last_histogram_ix_[0i32 as (usize)].wrapping_add(
                             i
                         ) as (isize)
                     ) = combined_histo[num_contexts.wrapping_add(i)];
                    *last_entropy.offset(
                         num_contexts.wrapping_add(i) as (isize)
                     ) = *last_entropy.offset(i as (isize));
                    *last_entropy.offset(i as (isize)) = combined_entropy[
                                                             num_contexts.wrapping_add(i)
                                                         ];
                    HistogramClearLiteral(
                        &mut *histograms.offset(
                                  (*self).curr_histogram_ix_.wrapping_add(i) as (isize)
                              ) as (*mut HistogramLiteral)
                    );
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop17;
                } else {
                    break 'loop17;
                }
            }
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*self).block_size_ = 0i32 as (usize);
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else {
            {
                let _rhs = (*self).block_size_ as (u32);
                let _lhs
                    = &mut *(*split).lengths.offset(
                                (*self).num_blocks_.wrapping_sub(1i32 as (usize)) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            i = 0i32 as (usize);
            'loop9: loop {
                if i < num_contexts {
                    *histograms.offset(
                         (*self).last_histogram_ix_[0i32 as (usize)].wrapping_add(
                             i
                         ) as (isize)
                     ) = combined_histo[i];
                    *last_entropy.offset(i as (isize)) = combined_entropy[i];
                    if (*split).num_types == 1i32 as (usize) {
                        *last_entropy.offset(
                             num_contexts.wrapping_add(i) as (isize)
                         ) = *last_entropy.offset(i as (isize));
                    }
                    HistogramClearLiteral(
                        &mut *histograms.offset(
                                  (*self).curr_histogram_ix_.wrapping_add(i) as (isize)
                              ) as (*mut HistogramLiteral)
                    );
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop9;
                } else {
                    break 'loop9;
                }
            }
            (*self).block_size_ = 0i32 as (usize);
            if {
                   (*self).merge_last_count_ = (*self).merge_last_count_.wrapping_add(
                                                   1 as (usize)
                                               );
                   (*self).merge_last_count_
               } > 1i32 as (usize) {
                (*self).target_block_size_ = (*self).target_block_size_.wrapping_add(
                                                 (*self).min_block_size_
                                             );
            }
        }
    }
    if is_final != 0 {
        *(*self).histograms_size_ = (*split).num_types.wrapping_mul(
                                        num_contexts
                                    );
        (*split).num_blocks = (*self).num_blocks_;
    }
}

unsafe extern fn ContextBlockSplitterAddSymbol(
    mut self : *mut ContextBlockSplitter,
    mut symbol : usize,
    mut context : usize
) {
    HistogramAddLiteral(
        &mut *(*self).histograms_.offset(
                  (*self).curr_histogram_ix_.wrapping_add(context) as (isize)
              ) as (*mut HistogramLiteral),
        symbol
    );
    (*self).block_size_ = (*self).block_size_.wrapping_add(
                              1 as (usize)
                          );
    if (*self).block_size_ == (*self).target_block_size_ {
        ContextBlockSplitterFinishBlock(self,0i32);
    }
}

unsafe extern fn BlockSplitterFinishBlockLiteral(
    mut self : *mut BlockSplitterLiteral, mut is_final : i32
) {
    let mut split : *mut BlockSplit = (*self).split_;
    let mut last_entropy
        : *mut f64
        = (*self).last_entropy_.as_mut_ptr();
    let mut histograms : *mut HistogramLiteral = (*self).histograms_;
    (*self).block_size_ = brotli_max_size_t(
                              (*self).block_size_,
                              (*self).min_block_size_
                          );
    if (*self).num_blocks_ == 0i32 as (usize) {
        *(*split).lengths.offset(
             0i32 as (isize)
         ) = (*self).block_size_ as (u32);
        *(*split).types.offset(0i32 as (isize)) = 0i32 as (u8);
        *last_entropy.offset(0i32 as (isize)) = BitsEntropy(
                                                    (*histograms.offset(
                                                          0i32 as (isize)
                                                      )).data_.as_mut_ptr(
                                                    ) as (*const u32),
                                                    (*self).alphabet_size_
                                                );
        *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                     0i32 as (isize)
                                                 );
        (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                  1 as (usize)
                              );
        (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
        (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                         1 as (usize)
                                     );
        if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
            HistogramClearLiteral(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramLiteral)
            );
        }
        (*self).block_size_ = 0i32 as (usize);
    } else if (*self).block_size_ > 0i32 as (usize) {
        let mut entropy
            : f64
            = BitsEntropy(
                  (*histograms.offset(
                        (*self).curr_histogram_ix_ as (isize)
                    )).data_.as_mut_ptr(
                  ) as (*const u32),
                  (*self).alphabet_size_
              );
        let mut combined_histo : [HistogramLiteral; 2];
        let mut combined_entropy : [f64; 2];
        let mut diff : [f64; 2];
        let mut j : usize;
        j = 0i32 as (usize);
        'loop3: loop {
            if j < 2i32 as (usize) {
                let mut last_histogram_ix : usize = (*self).last_histogram_ix_[j];
                combined_histo[j] = *histograms.offset(
                                         (*self).curr_histogram_ix_ as (isize)
                                     );
                HistogramAddHistogramLiteral(
                    &mut combined_histo[j] as (*mut HistogramLiteral),
                    &mut *histograms.offset(
                              last_histogram_ix as (isize)
                          ) as (*mut HistogramLiteral) as (*const HistogramLiteral)
                );
                combined_entropy[j] = BitsEntropy(
                                          &mut combined_histo[j].data_[
                                                   0i32 as (usize)
                                               ] as (*mut u32) as (*const u32),
                                          (*self).alphabet_size_
                                      );
                diff[j] = combined_entropy[j] - entropy - *last_entropy.offset(
                                                               j as (isize)
                                                           );
                j = j.wrapping_add(1 as (usize));
                continue 'loop3;
            } else {
                break 'loop3;
            }
        }
        if (*split).num_types < 256i32 as (usize) && (diff[
                                                          0i32 as (usize)
                                                      ] > (*self).split_threshold_) && (diff[
                                                                                            1i32 as (usize)
                                                                                        ] > (*self).split_threshold_) {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*split).num_types as (u8);
            (*self).last_histogram_ix_[
                1i32 as (usize)
            ] = (*self).last_histogram_ix_[0i32 as (usize)];
            (*self).last_histogram_ix_[
                0i32 as (usize)
            ] = (*split).num_types as (u8) as (usize);
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = entropy;
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
            (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                             1 as (usize)
                                         );
            if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
                HistogramClearLiteral(
                    &mut *histograms.offset(
                              (*self).curr_histogram_ix_ as (isize)
                          ) as (*mut HistogramLiteral)
                );
            }
            (*self).block_size_ = 0i32 as (usize);
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else if diff[1i32 as (usize)] < diff[0i32 as (usize)] - 20.0f64 {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = *(*split).types.offset(
                      (*self).num_blocks_.wrapping_sub(2i32 as (usize)) as (isize)
                  );
            let mut __brotli_swap_tmp
                : usize
                = (*self).last_histogram_ix_[0i32 as (usize)];
            (*self).last_histogram_ix_[
                0i32 as (usize)
            ] = (*self).last_histogram_ix_[1i32 as (usize)];
            (*self).last_histogram_ix_[1i32 as (usize)] = __brotli_swap_tmp;
            *histograms.offset(
                 (*self).last_histogram_ix_[0i32 as (usize)] as (isize)
             ) = combined_histo[1i32 as (usize)];
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = combined_entropy[
                                                        1i32 as (usize)
                                                    ];
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearLiteral(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramLiteral)
            );
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else {
            {
                let _rhs = (*self).block_size_ as (u32);
                let _lhs
                    = &mut *(*split).lengths.offset(
                                (*self).num_blocks_.wrapping_sub(1i32 as (usize)) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            *histograms.offset(
                 (*self).last_histogram_ix_[0i32 as (usize)] as (isize)
             ) = combined_histo[0i32 as (usize)];
            *last_entropy.offset(0i32 as (isize)) = combined_entropy[
                                                        0i32 as (usize)
                                                    ];
            if (*split).num_types == 1i32 as (usize) {
                *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                             0i32 as (isize)
                                                         );
            }
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearLiteral(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramLiteral)
            );
            if {
                   (*self).merge_last_count_ = (*self).merge_last_count_.wrapping_add(
                                                   1 as (usize)
                                               );
                   (*self).merge_last_count_
               } > 1i32 as (usize) {
                (*self).target_block_size_ = (*self).target_block_size_.wrapping_add(
                                                 (*self).min_block_size_
                                             );
            }
        }
    }
    if is_final != 0 {
        *(*self).histograms_size_ = (*split).num_types;
        (*split).num_blocks = (*self).num_blocks_;
    }
}

unsafe extern fn BlockSplitterAddSymbolLiteral(
    mut self : *mut BlockSplitterLiteral, mut symbol : usize
) {
    HistogramAddLiteral(
        &mut *(*self).histograms_.offset(
                  (*self).curr_histogram_ix_ as (isize)
              ) as (*mut HistogramLiteral),
        symbol
    );
    (*self).block_size_ = (*self).block_size_.wrapping_add(
                              1 as (usize)
                          );
    if (*self).block_size_ == (*self).target_block_size_ {
        BlockSplitterFinishBlockLiteral(self,0i32);
    }
}

unsafe extern fn CommandCopyLen(mut self : *const Command) -> u32 {
    (*self).copy_len_ & 0xffffffi32 as (u32)
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

unsafe extern fn BlockSplitterFinishBlockDistance(
    mut self : *mut BlockSplitterDistance, mut is_final : i32
) {
    let mut split : *mut BlockSplit = (*self).split_;
    let mut last_entropy
        : *mut f64
        = (*self).last_entropy_.as_mut_ptr();
    let mut histograms : *mut HistogramDistance = (*self).histograms_;
    (*self).block_size_ = brotli_max_size_t(
                              (*self).block_size_,
                              (*self).min_block_size_
                          );
    if (*self).num_blocks_ == 0i32 as (usize) {
        *(*split).lengths.offset(
             0i32 as (isize)
         ) = (*self).block_size_ as (u32);
        *(*split).types.offset(0i32 as (isize)) = 0i32 as (u8);
        *last_entropy.offset(0i32 as (isize)) = BitsEntropy(
                                                    (*histograms.offset(
                                                          0i32 as (isize)
                                                      )).data_.as_mut_ptr(
                                                    ) as (*const u32),
                                                    (*self).alphabet_size_
                                                );
        *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                     0i32 as (isize)
                                                 );
        (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                  1 as (usize)
                              );
        (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
        (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                         1 as (usize)
                                     );
        if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
            HistogramClearDistance(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramDistance)
            );
        }
        (*self).block_size_ = 0i32 as (usize);
    } else if (*self).block_size_ > 0i32 as (usize) {
        let mut entropy
            : f64
            = BitsEntropy(
                  (*histograms.offset(
                        (*self).curr_histogram_ix_ as (isize)
                    )).data_.as_mut_ptr(
                  ) as (*const u32),
                  (*self).alphabet_size_
              );
        let mut combined_histo : [HistogramDistance; 2];
        let mut combined_entropy : [f64; 2];
        let mut diff : [f64; 2];
        let mut j : usize;
        j = 0i32 as (usize);
        'loop3: loop {
            if j < 2i32 as (usize) {
                let mut last_histogram_ix : usize = (*self).last_histogram_ix_[j];
                combined_histo[j] = *histograms.offset(
                                         (*self).curr_histogram_ix_ as (isize)
                                     );
                HistogramAddHistogramDistance(
                    &mut combined_histo[j] as (*mut HistogramDistance),
                    &mut *histograms.offset(
                              last_histogram_ix as (isize)
                          ) as (*mut HistogramDistance) as (*const HistogramDistance)
                );
                combined_entropy[j] = BitsEntropy(
                                          &mut combined_histo[j].data_[
                                                   0i32 as (usize)
                                               ] as (*mut u32) as (*const u32),
                                          (*self).alphabet_size_
                                      );
                diff[j] = combined_entropy[j] - entropy - *last_entropy.offset(
                                                               j as (isize)
                                                           );
                j = j.wrapping_add(1 as (usize));
                continue 'loop3;
            } else {
                break 'loop3;
            }
        }
        if (*split).num_types < 256i32 as (usize) && (diff[
                                                          0i32 as (usize)
                                                      ] > (*self).split_threshold_) && (diff[
                                                                                            1i32 as (usize)
                                                                                        ] > (*self).split_threshold_) {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*split).num_types as (u8);
            (*self).last_histogram_ix_[
                1i32 as (usize)
            ] = (*self).last_histogram_ix_[0i32 as (usize)];
            (*self).last_histogram_ix_[
                0i32 as (usize)
            ] = (*split).num_types as (u8) as (usize);
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = entropy;
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
            (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                             1 as (usize)
                                         );
            if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
                HistogramClearDistance(
                    &mut *histograms.offset(
                              (*self).curr_histogram_ix_ as (isize)
                          ) as (*mut HistogramDistance)
                );
            }
            (*self).block_size_ = 0i32 as (usize);
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else if diff[1i32 as (usize)] < diff[0i32 as (usize)] - 20.0f64 {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = *(*split).types.offset(
                      (*self).num_blocks_.wrapping_sub(2i32 as (usize)) as (isize)
                  );
            let mut __brotli_swap_tmp
                : usize
                = (*self).last_histogram_ix_[0i32 as (usize)];
            (*self).last_histogram_ix_[
                0i32 as (usize)
            ] = (*self).last_histogram_ix_[1i32 as (usize)];
            (*self).last_histogram_ix_[1i32 as (usize)] = __brotli_swap_tmp;
            *histograms.offset(
                 (*self).last_histogram_ix_[0i32 as (usize)] as (isize)
             ) = combined_histo[1i32 as (usize)];
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = combined_entropy[
                                                        1i32 as (usize)
                                                    ];
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearDistance(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramDistance)
            );
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else {
            {
                let _rhs = (*self).block_size_ as (u32);
                let _lhs
                    = &mut *(*split).lengths.offset(
                                (*self).num_blocks_.wrapping_sub(1i32 as (usize)) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            *histograms.offset(
                 (*self).last_histogram_ix_[0i32 as (usize)] as (isize)
             ) = combined_histo[0i32 as (usize)];
            *last_entropy.offset(0i32 as (isize)) = combined_entropy[
                                                        0i32 as (usize)
                                                    ];
            if (*split).num_types == 1i32 as (usize) {
                *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                             0i32 as (isize)
                                                         );
            }
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearDistance(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramDistance)
            );
            if {
                   (*self).merge_last_count_ = (*self).merge_last_count_.wrapping_add(
                                                   1 as (usize)
                                               );
                   (*self).merge_last_count_
               } > 1i32 as (usize) {
                (*self).target_block_size_ = (*self).target_block_size_.wrapping_add(
                                                 (*self).min_block_size_
                                             );
            }
        }
    }
    if is_final != 0 {
        *(*self).histograms_size_ = (*split).num_types;
        (*split).num_blocks = (*self).num_blocks_;
    }
}

unsafe extern fn BlockSplitterAddSymbolDistance(
    mut self : *mut BlockSplitterDistance, mut symbol : usize
) {
    HistogramAddDistance(
        &mut *(*self).histograms_.offset(
                  (*self).curr_histogram_ix_ as (isize)
              ) as (*mut HistogramDistance),
        symbol
    );
    (*self).block_size_ = (*self).block_size_.wrapping_add(
                              1 as (usize)
                          );
    if (*self).block_size_ == (*self).target_block_size_ {
        BlockSplitterFinishBlockDistance(self,0i32);
    }
}

unsafe extern fn MapStaticContexts(
    mut m : *mut MemoryManager,
    mut num_contexts : usize,
    mut static_context_map : *const u32,
    mut mb : *mut MetaBlockSplit
) {
    let mut i : usize;
    if (*mb).literal_context_map == 0i32 as (*mut u32) {
        0i32;
    } else {
        __assert_fail(
            (*b"mb->literal_context_map == 0\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"MapStaticContexts\0").as_ptr()
        );
    }
    (*mb).literal_context_map_size = (*mb).literal_split.num_types << 6i32;
    (*mb).literal_context_map = if (*mb).literal_context_map_size != 0 {
                                    BrotliAllocate(
                                        m,
                                        (*mb).literal_context_map_size.wrapping_mul(
                                            ::std::mem::size_of::<u32>()
                                        )
                                    ) as (*mut u32)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                };
    if !(0i32 == 0) {
    } else {
        i = 0i32 as (usize);
        'loop2: loop {
            if i < (*mb).literal_split.num_types {
                let mut offset : u32 = i.wrapping_mul(num_contexts) as (u32);
                let mut j : usize;
                j = 0i32 as (usize);
                'loop5: loop {
                    if j < (1u32 << 6i32) as (usize) {
                        *(*mb).literal_context_map.offset(
                             (i << 6i32).wrapping_add(j) as (isize)
                         ) = offset.wrapping_add(*static_context_map.offset(j as (isize)));
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop5;
                    } else {
                        break 'loop5;
                    }
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
    }
}

unsafe extern fn BrotliBuildMetaBlockGreedyInternal(
    mut m : *mut MemoryManager,
    mut ringbuffer : *const u8,
    mut pos : usize,
    mut mask : usize,
    mut prev_byte : u8,
    mut prev_byte2 : u8,
    mut literal_context_mode : ContextType,
    num_contexts : usize,
    mut static_context_map : *const u32,
    mut commands : *const Command,
    mut n_commands : usize,
    mut mb : *mut MetaBlockSplit
) {
    let mut lit_blocks : LitBlocks;
    let mut cmd_blocks : BlockSplitterCommand;
    let mut dist_blocks : BlockSplitterDistance;
    let mut num_literals : usize = 0i32 as (usize);
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < n_commands {
            num_literals = num_literals.wrapping_add(
                               (*commands.offset(i as (isize))).insert_len_ as (usize)
                           );
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    if num_contexts == 1i32 as (usize) {
        InitBlockSplitterLiteral(
            m,
            &mut lit_blocks.plain as (*mut BlockSplitterLiteral),
            256i32 as (usize),
            512i32 as (usize),
            400.0f64,
            num_literals,
            &mut (*mb).literal_split as (*mut BlockSplit),
            &mut (*mb).literal_histograms as (*mut *mut HistogramLiteral),
            &mut (*mb).literal_histograms_size as (*mut usize)
        );
    } else {
        InitContextBlockSplitter(
            m,
            &mut lit_blocks.ctx as (*mut ContextBlockSplitter),
            256i32 as (usize),
            num_contexts,
            512i32 as (usize),
            400.0f64,
            num_literals,
            &mut (*mb).literal_split as (*mut BlockSplit),
            &mut (*mb).literal_histograms as (*mut *mut HistogramLiteral),
            &mut (*mb).literal_histograms_size as (*mut usize)
        );
    }
    if !(0i32 == 0) {
    } else {
        InitBlockSplitterCommand(
            m,
            &mut cmd_blocks as (*mut BlockSplitterCommand),
            704i32 as (usize),
            1024i32 as (usize),
            500.0f64,
            n_commands,
            &mut (*mb).command_split as (*mut BlockSplit),
            &mut (*mb).command_histograms as (*mut *mut HistogramCommand),
            &mut (*mb).command_histograms_size as (*mut usize)
        );
        if !(0i32 == 0) {
        } else {
            InitBlockSplitterDistance(
                m,
                &mut dist_blocks as (*mut BlockSplitterDistance),
                64i32 as (usize),
                512i32 as (usize),
                100.0f64,
                n_commands,
                &mut (*mb).distance_split as (*mut BlockSplit),
                &mut (*mb).distance_histograms as (*mut *mut HistogramDistance),
                &mut (*mb).distance_histograms_size as (*mut usize)
            );
            if !(0i32 == 0) {
            } else {
                i = 0i32 as (usize);
                'loop9: loop {
                    if i < n_commands {
                        let cmd : Command = *commands.offset(i as (isize));
                        let mut j : usize;
                        BlockSplitterAddSymbolCommand(
                            &mut cmd_blocks as (*mut BlockSplitterCommand),
                            cmd.cmd_prefix_ as (usize)
                        );
                        j = cmd.insert_len_ as (usize);
                        'loop17: loop {
                            if j != 0i32 as (usize) {
                                let mut literal : u8 = *ringbuffer.offset((pos & mask) as (isize));
                                if num_contexts == 1i32 as (usize) {
                                    BlockSplitterAddSymbolLiteral(
                                        &mut lit_blocks.plain as (*mut BlockSplitterLiteral),
                                        literal as (usize)
                                    );
                                } else {
                                    let mut context
                                        : usize
                                        = Context(
                                              prev_byte,
                                              prev_byte2,
                                              literal_context_mode
                                          ) as (usize);
                                    ContextBlockSplitterAddSymbol(
                                        &mut lit_blocks.ctx as (*mut ContextBlockSplitter),
                                        literal as (usize),
                                        *static_context_map.offset(context as (isize)) as (usize)
                                    );
                                }
                                prev_byte2 = prev_byte;
                                prev_byte = literal;
                                pos = pos.wrapping_add(1 as (usize));
                                j = j.wrapping_sub(1 as (usize));
                                continue 'loop17;
                            } else {
                                break 'loop17;
                            }
                        }
                        pos = pos.wrapping_add(
                                  CommandCopyLen(&cmd as (*const Command)) as (usize)
                              );
                        if CommandCopyLen(&cmd as (*const Command)) != 0 {
                            prev_byte2 = *ringbuffer.offset(
                                              (pos.wrapping_sub(2i32 as (usize)) & mask) as (isize)
                                          );
                            prev_byte = *ringbuffer.offset(
                                             (pos.wrapping_sub(1i32 as (usize)) & mask) as (isize)
                                         );
                            if cmd.cmd_prefix_ as (i32) >= 128i32 {
                                BlockSplitterAddSymbolDistance(
                                    &mut dist_blocks as (*mut BlockSplitterDistance),
                                    cmd.dist_prefix_ as (usize)
                                );
                            }
                        }
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop9;
                    } else {
                        break 'loop9;
                    }
                }
                if num_contexts == 1i32 as (usize) {
                    BlockSplitterFinishBlockLiteral(
                        &mut lit_blocks.plain as (*mut BlockSplitterLiteral),
                        1i32
                    );
                } else {
                    ContextBlockSplitterFinishBlock(
                        &mut lit_blocks.ctx as (*mut ContextBlockSplitter),
                        1i32
                    );
                }
                BlockSplitterFinishBlockCommand(
                    &mut cmd_blocks as (*mut BlockSplitterCommand),
                    1i32
                );
                BlockSplitterFinishBlockDistance(
                    &mut dist_blocks as (*mut BlockSplitterDistance),
                    1i32
                );
                if num_contexts > 1i32 as (usize) {
                    MapStaticContexts(m,num_contexts,static_context_map,mb);
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliBuildMetaBlockGreedy(
    mut m : *mut MemoryManager,
    mut ringbuffer : *const u8,
    mut pos : usize,
    mut mask : usize,
    mut prev_byte : u8,
    mut prev_byte2 : u8,
    mut literal_context_mode : ContextType,
    mut num_contexts : usize,
    mut static_context_map : *const u32,
    mut commands : *const Command,
    mut n_commands : usize,
    mut mb : *mut MetaBlockSplit
) { if num_contexts == 1i32 as (usize) {
        BrotliBuildMetaBlockGreedyInternal(
            m,
            ringbuffer,
            pos,
            mask,
            prev_byte,
            prev_byte2,
            literal_context_mode,
            1i32 as (usize),
            0i32 as (*mut ::std::os::raw::c_void) as (*const u32),
            commands,
            n_commands,
            mb
        );
    } else {
        BrotliBuildMetaBlockGreedyInternal(
            m,
            ringbuffer,
            pos,
            mask,
            prev_byte,
            prev_byte2,
            literal_context_mode,
            num_contexts,
            static_context_map,
            commands,
            n_commands,
            mb
        );
    }
}

#[no_mangle]
pub unsafe extern fn BrotliOptimizeHistograms(
    mut num_direct_distance_codes : usize,
    mut distance_postfix_bits : usize,
    mut mb : *mut MetaBlockSplit
) {
    let mut good_for_rle : [u8; 704];
    let mut num_distance_codes : usize;
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < (*mb).literal_histograms_size {
            BrotliOptimizeHuffmanCountsForRle(
                256i32 as (usize),
                (*(*mb).literal_histograms.offset(i as (isize))).data_.as_mut_ptr(
                ),
                good_for_rle.as_mut_ptr()
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    i = 0i32 as (usize);
    'loop3: loop {
        if i < (*mb).command_histograms_size {
            BrotliOptimizeHuffmanCountsForRle(
                704i32 as (usize),
                (*(*mb).command_histograms.offset(i as (isize))).data_.as_mut_ptr(
                ),
                good_for_rle.as_mut_ptr()
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    num_distance_codes = (16i32 as (usize)).wrapping_add(
                             num_direct_distance_codes
                         ).wrapping_add(
                             ((2i32 as (u32)).wrapping_mul(
                                  24u32
                              ) << distance_postfix_bits) as (usize)
                         );
    i = 0i32 as (usize);
    'loop5: loop {
        if i < (*mb).distance_histograms_size {
            BrotliOptimizeHuffmanCountsForRle(
                num_distance_codes,
                (*(*mb).distance_histograms.offset(i as (isize))).data_.as_mut_ptr(
                ),
                good_for_rle.as_mut_ptr()
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop5;
        } else {
            break 'loop5;
        }
    }
}
