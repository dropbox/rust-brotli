extern {
    fn BrotliAllocate(
        m : *mut MemoryManager, n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn BrotliBuildMetaBlock(
        m : *mut MemoryManager,
        ringbuffer : *const u8,
        pos : usize,
        mask : usize,
        params : *const BrotliEncoderParams,
        prev_byte : u8,
        prev_byte2 : u8,
        cmds : *const Command,
        num_commands : usize,
        literal_context_mode : ContextType,
        mb : *mut MetaBlockSplit
    );
    fn BrotliBuildMetaBlockGreedy(
        m : *mut MemoryManager,
        ringbuffer : *const u8,
        pos : usize,
        mask : usize,
        prev_byte : u8,
        prev_byte2 : u8,
        literal_context_mode : ContextType,
        num_contexts : usize,
        static_context_map : *const u32,
        commands : *const Command,
        n_commands : usize,
        mb : *mut MetaBlockSplit
    );
    fn BrotliCompressFragmentFast(
        m : *mut MemoryManager,
        input : *const u8,
        input_size : usize,
        is_last : i32,
        table : *mut i32,
        table_size : usize,
        cmd_depth : *mut u8,
        cmd_bits : *mut u16,
        cmd_code_numbits : *mut usize,
        cmd_code : *mut u8,
        storage_ix : *mut usize,
        storage : *mut u8
    );
    fn BrotliCompressFragmentTwoPass(
        m : *mut MemoryManager,
        input : *const u8,
        input_size : usize,
        is_last : i32,
        command_buf : *mut u32,
        literal_buf : *mut u8,
        table : *mut i32,
        table_size : usize,
        storage_ix : *mut usize,
        storage : *mut u8
    );
    fn BrotliCreateBackwardReferences(
        dictionary : *const BrotliDictionary,
        num_bytes : usize,
        position : usize,
        ringbuffer : *const u8,
        ringbuffer_mask : usize,
        params : *const BrotliEncoderParams,
        hasher : *mut u8,
        dist_cache : *mut i32,
        last_insert_len : *mut usize,
        commands : *mut Command,
        num_commands : *mut usize,
        num_literals : *mut usize
    );
    fn BrotliCreateHqZopfliBackwardReferences(
        m : *mut MemoryManager,
        dictionary : *const BrotliDictionary,
        num_bytes : usize,
        position : usize,
        ringbuffer : *const u8,
        ringbuffer_mask : usize,
        params : *const BrotliEncoderParams,
        hasher : *mut u8,
        dist_cache : *mut i32,
        last_insert_len : *mut usize,
        commands : *mut Command,
        num_commands : *mut usize,
        num_literals : *mut usize
    );
    fn BrotliCreateZopfliBackwardReferences(
        m : *mut MemoryManager,
        dictionary : *const BrotliDictionary,
        num_bytes : usize,
        position : usize,
        ringbuffer : *const u8,
        ringbuffer_mask : usize,
        params : *const BrotliEncoderParams,
        hasher : *mut u8,
        dist_cache : *mut i32,
        last_insert_len : *mut usize,
        commands : *mut Command,
        num_commands : *mut usize,
        num_literals : *mut usize
    );
    fn BrotliDestroyBlockSplit(
        m : *mut MemoryManager, self : *mut BlockSplit
    );
    fn BrotliFree(
        m : *mut MemoryManager, p : *mut ::std::os::raw::c_void
    );
    fn BrotliGetDictionary() -> *const BrotliDictionary;
    fn BrotliInitBlockSplit(self : *mut BlockSplit);
    fn BrotliInitMemoryManager(
        m : *mut MemoryManager,
        alloc_func
        :
        unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void,
        free_func
        :
        unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
        opaque : *mut ::std::os::raw::c_void
    );
    fn BrotliInitZopfliNodes(array : *mut ZopfliNode, length : usize);
    fn BrotliIsMostlyUTF8(
        data : *const u8,
        pos : usize,
        mask : usize,
        length : usize,
        min_fraction : f64
    ) -> i32;
    fn BrotliOptimizeHistograms(
        num_direct_distance_codes : usize,
        distance_postfix_bits : usize,
        mb : *mut MetaBlockSplit
    );
    fn BrotliStoreMetaBlock(
        m : *mut MemoryManager,
        input : *const u8,
        start_pos : usize,
        length : usize,
        mask : usize,
        prev_byte : u8,
        prev_byte2 : u8,
        is_final_block : i32,
        num_direct_distance_codes : u32,
        distance_postfix_bits : u32,
        literal_context_mode : ContextType,
        commands : *const Command,
        n_commands : usize,
        mb : *const MetaBlockSplit,
        storage_ix : *mut usize,
        storage : *mut u8
    );
    fn BrotliStoreMetaBlockFast(
        m : *mut MemoryManager,
        input : *const u8,
        start_pos : usize,
        length : usize,
        mask : usize,
        is_last : i32,
        commands : *const Command,
        n_commands : usize,
        storage_ix : *mut usize,
        storage : *mut u8
    );
    fn BrotliStoreMetaBlockTrivial(
        m : *mut MemoryManager,
        input : *const u8,
        start_pos : usize,
        length : usize,
        mask : usize,
        is_last : i32,
        commands : *const Command,
        n_commands : usize,
        storage_ix : *mut usize,
        storage : *mut u8
    );
    fn BrotliStoreUncompressedMetaBlock(
        is_final_block : i32,
        input : *const u8,
        position : usize,
        mask : usize,
        len : usize,
        storage_ix : *mut usize,
        storage : *mut u8
    );
    fn BrotliWipeOutMemoryManager(m : *mut MemoryManager);
    fn BrotliZopfliComputeShortestPath(
        m : *mut MemoryManager,
        dictionary : *const BrotliDictionary,
        num_bytes : usize,
        position : usize,
        ringbuffer : *const u8,
        ringbuffer_mask : usize,
        params : *const BrotliEncoderParams,
        max_backward_limit : usize,
        dist_cache : *const i32,
        hasher : *mut u8,
        nodes : *mut ZopfliNode
    ) -> usize;
    fn BrotliZopfliCreateCommands(
        num_bytes : usize,
        block_start : usize,
        max_backward_limit : usize,
        nodes : *const ZopfliNode,
        dist_cache : *mut i32,
        last_insert_len : *mut usize,
        commands : *mut Command,
        num_literals : *mut usize
    );
    fn __swbuf(arg1 : i32, arg2 : *mut __sFILE) -> i32;
    fn malloc(__size : usize) -> *mut ::std::os::raw::c_void;
    fn memcpy(
        __dst : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __b : *mut ::std::os::raw::c_void, __c : i32, __len : usize
    ) -> *mut ::std::os::raw::c_void;
}

enum __sFILEX {
}

static kBrotliMinWindowBits : i32 = 10i32;

static kBrotliMaxWindowBits : i32 = 24i32;

static mut kLog2Table
    : [f64; 256]
    = [   0.0000000000000000f64,
          0.0000000000000000f64,
          1.0000000000000000f64,
          1.5849625007211563f64,
          2.0000000000000000f64,
          2.3219280948873622f64,
          2.5849625007211561f64,
          2.8073549220576042f64,
          3.0000000000000000f64,
          3.1699250014423126f64,
          3.3219280948873626f64,
          3.4594316186372978f64,
          3.5849625007211565f64,
          3.7004397181410922f64,
          3.8073549220576037f64,
          3.9068905956085187f64,
          4.0000000000000000f64,
          4.0874628412503400f64,
          4.1699250014423122f64,
          4.2479275134435852f64,
          4.3219280948873626f64,
          4.3923174227787607f64,
          4.4594316186372973f64,
          4.5235619560570131f64,
          4.5849625007211570f64,
          4.6438561897747244f64,
          4.7004397181410926f64,
          4.7548875021634691f64,
          4.8073549220576037f64,
          4.8579809951275728f64,
          4.9068905956085187f64,
          4.9541963103868758f64,
          5.0000000000000000f64,
          5.0443941193584534f64,
          5.0874628412503400f64,
          5.1292830169449664f64,
          5.1699250014423122f64,
          5.2094533656289501f64,
          5.2479275134435852f64,
          5.2854022188622487f64,
          5.3219280948873626f64,
          5.3575520046180838f64,
          5.3923174227787607f64,
          5.4262647547020979f64,
          5.4594316186372973f64,
          5.4918530963296748f64,
          5.5235619560570131f64,
          5.5545888516776376f64,
          5.5849625007211570f64,
          5.6147098441152083f64,
          5.6438561897747244f64,
          5.6724253419714961f64,
          5.7004397181410926f64,
          5.7279204545631996f64,
          5.7548875021634691f64,
          5.7813597135246599f64,
          5.8073549220576046f64,
          5.8328900141647422f64,
          5.8579809951275719f64,
          5.8826430493618416f64,
          5.9068905956085187f64,
          5.9307373375628867f64,
          5.9541963103868758f64,
          5.9772799234999168f64,
          6.0000000000000000f64,
          6.0223678130284544f64,
          6.0443941193584534f64,
          6.0660891904577721f64,
          6.0874628412503400f64,
          6.1085244567781700f64,
          6.1292830169449672f64,
          6.1497471195046822f64,
          6.1699250014423122f64,
          6.1898245588800176f64,
          6.2094533656289510f64,
          6.2288186904958804f64,
          6.2479275134435861f64,
          6.2667865406949019f64,
          6.2854022188622487f64,
          6.3037807481771031f64,
          6.3219280948873617f64,
          6.3398500028846252f64,
          6.3575520046180847f64,
          6.3750394313469254f64,
          6.3923174227787598f64,
          6.4093909361377026f64,
          6.4262647547020979f64,
          6.4429434958487288f64,
          6.4594316186372982f64,
          6.4757334309663976f64,
          6.4918530963296748f64,
          6.5077946401986964f64,
          6.5235619560570131f64,
          6.5391588111080319f64,
          6.5545888516776376f64,
          6.5698556083309478f64,
          6.5849625007211561f64,
          6.5999128421871278f64,
          6.6147098441152092f64,
          6.6293566200796095f64,
          6.6438561897747253f64,
          6.6582114827517955f64,
          6.6724253419714952f64,
          6.6865005271832185f64,
          6.7004397181410917f64,
          6.7142455176661224f64,
          6.7279204545631988f64,
          6.7414669864011465f64,
          6.7548875021634691f64,
          6.7681843247769260f64,
          6.7813597135246599f64,
          6.7944158663501062f64,
          6.8073549220576037f64,
          6.8201789624151887f64,
          6.8328900141647422f64,
          6.8454900509443757f64,
          6.8579809951275719f64,
          6.8703647195834048f64,
          6.8826430493618416f64,
          6.8948177633079437f64,
          6.9068905956085187f64,
          6.9188632372745955f64,
          6.9307373375628867f64,
          6.9425145053392399f64,
          6.9541963103868758f64,
          6.9657842846620879f64,
          6.9772799234999168f64,
          6.9886846867721664f64,
          7.0000000000000000f64,
          7.0112272554232540f64,
          7.0223678130284544f64,
          7.0334230015374501f64,
          7.0443941193584534f64,
          7.0552824355011898f64,
          7.0660891904577721f64,
          7.0768155970508317f64,
          7.0874628412503400f64,
          7.0980320829605272f64,
          7.1085244567781700f64,
          7.1189410727235076f64,
          7.1292830169449664f64,
          7.1395513523987937f64,
          7.1497471195046822f64,
          7.1598713367783891f64,
          7.1699250014423130f64,
          7.1799090900149345f64,
          7.1898245588800176f64,
          7.1996723448363644f64,
          7.2094533656289492f64,
          7.2191685204621621f64,
          7.2288186904958804f64,
          7.2384047393250794f64,
          7.2479275134435861f64,
          7.2573878426926521f64,
          7.2667865406949019f64,
          7.2761244052742384f64,
          7.2854022188622487f64,
          7.2946207488916270f64,
          7.3037807481771031f64,
          7.3128829552843557f64,
          7.3219280948873617f64,
          7.3309168781146177f64,
          7.3398500028846243f64,
          7.3487281542310781f64,
          7.3575520046180847f64,
          7.3663222142458151f64,
          7.3750394313469254f64,
          7.3837042924740528f64,
          7.3923174227787607f64,
          7.4008794362821844f64,
          7.4093909361377026f64,
          7.4178525148858991f64,
          7.4262647547020979f64,
          7.4346282276367255f64,
          7.4429434958487288f64,
          7.4512111118323299f64,
          7.4594316186372973f64,
          7.4676055500829976f64,
          7.4757334309663976f64,
          7.4838157772642564f64,
          7.4918530963296748f64,
          7.4998458870832057f64,
          7.5077946401986964f64,
          7.5156998382840436f64,
          7.5235619560570131f64,
          7.5313814605163119f64,
          7.5391588111080319f64,
          7.5468944598876373f64,
          7.5545888516776376f64,
          7.5622424242210728f64,
          7.5698556083309478f64,
          7.5774288280357487f64,
          7.5849625007211561f64,
          7.5924570372680806f64,
          7.5999128421871278f64,
          7.6073303137496113f64,
          7.6147098441152075f64,
          7.6220518194563764f64,
          7.6293566200796095f64,
          7.6366246205436488f64,
          7.6438561897747244f64,
          7.6510516911789290f64,
          7.6582114827517955f64,
          7.6653359171851765f64,
          7.6724253419714952f64,
          7.6794800995054464f64,
          7.6865005271832185f64,
          7.6934869574993252f64,
          7.7004397181410926f64,
          7.7073591320808825f64,
          7.7142455176661224f64,
          7.7210991887071856f64,
          7.7279204545631996f64,
          7.7347096202258392f64,
          7.7414669864011465f64,
          7.7481928495894596f64,
          7.7548875021634691f64,
          7.7615512324444795f64,
          7.7681843247769260f64,
          7.7747870596011737f64,
          7.7813597135246608f64,
          7.7879025593914317f64,
          7.7944158663501062f64,
          7.8008998999203047f64,
          7.8073549220576037f64,
          7.8137811912170374f64,
          7.8201789624151887f64,
          7.8265484872909159f64,
          7.8328900141647422f64,
          7.8392037880969445f64,
          7.8454900509443757f64,
          7.8517490414160571f64,
          7.8579809951275719f64,
          7.8641861446542798f64,
          7.8703647195834048f64,
          7.8765169465650002f64,
          7.8826430493618425f64,
          7.8887432488982601f64,
          7.8948177633079446f64,
          7.9008668079807496f64,
          7.9068905956085187f64,
          7.9128893362299619f64,
          7.9188632372745955f64,
          7.9248125036057813f64,
          7.9307373375628867f64,
          7.9366379390025719f64,
          7.9425145053392399f64,
          7.9483672315846778f64,
          7.9541963103868758f64,
          7.9600019320680806f64,
          7.9657842846620870f64,
          7.9715435539507720f64,
          7.9772799234999168f64,
          7.9829935746943104f64,
          7.9886846867721664f64,
          7.9943534368588578f64
      ];

#[no_mangle]
pub unsafe extern fn log2(mut v : f64) -> f64 {
    if v < 0i32 as (f64) {
        0i32 as (f64)
    } else if v < 256i32 as (f64) {
        kLog2Table[v as (usize)]
    } else {
        let mut count : f64 = 0i32 as (f64);
        while 1i32 != 0 {
            v = v / 2i32 as (f64);
            count = count + 1.0f64;
            if v < 256i32 as (f64) {
                return kLog2Table[v as (usize)] + count;
            }
        }
    }
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

static kInvalidMatch : u32 = 0xfffffffi32 as (u32);

static kCutoffTransformsCount : u32 = 10i32 as (u32);

static kCutoffTransforms
    : usize
    = 0x71b520ai32 as (usize) << 32i32 | 0xda2d3200u32 as (usize);

static kHashMul32 : u32 = 0x1e35a7bdi32 as (u32);

static kHashMul64
    : usize
    = 0x1e35a7bdi32 as (usize) << 32i32 | 0x1e35a7bdi32 as (usize);

static kHashMul64Long
    : usize
    = 0x1fe35a7bu32 as (usize) << 32i32 | 0xd3579bd3u32 as (usize);

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

static kCompressFragmentTwoPassBlockSize
    : usize
    = (1i32 << 17i32) as (usize);

static kMinUTF8Ratio : f64 = 0.75f64;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct __sbuf {
    pub _base : *mut u8,
    pub _size : i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct __sFILE {
    pub _p : *mut u8,
    pub _r : i32,
    pub _w : i32,
    pub _flags : i16,
    pub _file : i16,
    pub _bf : __sbuf,
    pub _lbfsize : i32,
    pub _cookie : *mut ::std::os::raw::c_void,
    pub _close : unsafe extern fn(*mut ::std::os::raw::c_void) -> i32,
    pub _read : unsafe extern fn(*mut ::std::os::raw::c_void, *mut u8, i32) -> i32,
    pub _seek : unsafe extern fn(*mut ::std::os::raw::c_void, isize, i32) -> isize,
    pub _write : unsafe extern fn(*mut ::std::os::raw::c_void, *const u8, i32) -> i32,
    pub _ub : __sbuf,
    pub _extra : *mut __sFILEX,
    pub _ur : i32,
    pub _ubuf : [u8; 3],
    pub _nbuf : [u8; 1],
    pub _lb : __sbuf,
    pub _blksize : i32,
    pub _offset : isize,
}

#[no_mangle]
pub unsafe extern fn __sputc(
    mut _c : i32, mut _p : *mut __sFILE
) -> i32 {
    if {
           (*_p)._w = (*_p)._w - 1;
           (*_p)._w
       } >= 0i32 || (*_p)._w >= (*_p)._lbfsize && (_c as (u8) as (i32) != b'\n' as (i32)) {
        {
            let _rhs = _c;
            let _lhs
                = &mut *{
                            let _old = (*_p)._p;
                            (*_p)._p = (*_p)._p.offset(1 as (isize));
                            _old
                        };
            *_lhs = _rhs as (u8);
            *_lhs
        } as (i32)
    } else {
        __swbuf(_c,_p)
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum BrotliEncoderParameter {
    BROTLI_PARAM_MODE = 0i32,
    BROTLI_PARAM_QUALITY = 1i32,
    BROTLI_PARAM_LGWIN = 2i32,
    BROTLI_PARAM_LGBLOCK = 3i32,
    BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING = 4i32,
    BROTLI_PARAM_SIZE_HINT = 5i32,
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
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
    pub opaque : *mut ::std::os::raw::c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RingBuffer {
    pub size_ : u32,
    pub mask_ : u32,
    pub tail_size_ : u32,
    pub total_size_ : u32,
    pub cur_size_ : u32,
    pub pos_ : u32,
    pub data_ : *mut u8,
    pub buffer_index : usize,
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
#[repr(C)]
pub struct Struct1 {
    pub u8 : [u8; 16],
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum BrotliEncoderStreamState {
    BROTLI_STREAM_PROCESSING = 0i32,
    BROTLI_STREAM_FLUSH_REQUESTED = 1i32,
    BROTLI_STREAM_FINISHED = 2i32,
    BROTLI_STREAM_METADATA_HEAD = 3i32,
    BROTLI_STREAM_METADATA_BODY = 4i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliEncoderStateStruct {
    pub params : BrotliEncoderParams,
    pub memory_manager_ : MemoryManager,
    pub hasher_ : *mut u8,
    pub input_pos_ : usize,
    pub ringbuffer_ : RingBuffer,
    pub cmd_alloc_size_ : usize,
    pub commands_ : *mut Command,
    pub num_commands_ : usize,
    pub num_literals_ : usize,
    pub last_insert_len_ : usize,
    pub last_flush_pos_ : usize,
    pub last_processed_pos_ : usize,
    pub dist_cache_ : [i32; 16],
    pub saved_dist_cache_ : [i32; 4],
    pub last_byte_ : u8,
    pub last_byte_bits_ : u8,
    pub prev_byte_ : u8,
    pub prev_byte2_ : u8,
    pub storage_size_ : usize,
    pub storage_ : *mut u8,
    pub small_table_ : [i32; 1024],
    pub large_table_ : *mut i32,
    pub large_table_size_ : usize,
    pub cmd_depths_ : [u8; 128],
    pub cmd_bits_ : [u16; 128],
    pub cmd_code_ : [u8; 512],
    pub cmd_code_numbits_ : usize,
    pub command_buf_ : *mut u32,
    pub literal_buf_ : *mut u8,
    pub next_out_ : *mut u8,
    pub available_out_ : usize,
    pub total_out_ : usize,
    pub tiny_buf_ : Struct1,
    pub remaining_metadata_bytes_ : u32,
    pub stream_state_ : BrotliEncoderStreamState,
    pub is_last_block_emitted_ : i32,
    pub is_initialized_ : i32,
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderSetParameter(
    mut state : *mut BrotliEncoderStateStruct,
    mut p : BrotliEncoderParameter,
    mut value : u32
) -> i32 {
    if (*state).is_initialized_ != 0 {
        return 0i32;
    }
    if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_MODE as (i32) {
        (*state).params.mode = value as (BrotliEncoderMode);
        return 1i32;
    }
    if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_QUALITY as (i32) {
        (*state).params.quality = value as (i32);
        return 1i32;
    }
    if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_LGWIN as (i32) {
        (*state).params.lgwin = value as (i32);
        return 1i32;
    }
    if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_LGBLOCK as (i32) {
        (*state).params.lgblock = value as (i32);
        return 1i32;
    }
    if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING as (i32) {
        if value != 0i32 as (u32) && (value != 1i32 as (u32)) {
            return 0i32;
        }
        (*state).params.disable_literal_context_modeling = if !!!(value == 0) {
                                                               1i32
                                                           } else {
                                                               0i32
                                                           };
        return 1i32;
    }
    if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT as (i32) {
        (*state).params.size_hint = value as (usize);
        return 1i32;
    }
    0i32
}

unsafe extern fn BrotliEncoderInitParams(
    mut params : *mut BrotliEncoderParams
) {
    (*params).mode = BrotliEncoderMode::BROTLI_MODE_GENERIC;
    (*params).quality = 11i32;
    (*params).lgwin = 22i32;
    (*params).lgblock = 0i32;
    (*params).size_hint = 0i32 as (usize);
    (*params).disable_literal_context_modeling = 0i32;
}

unsafe extern fn RingBufferInit(mut rb : *mut RingBuffer) {
    (*rb).cur_size_ = 0i32 as (u32);
    (*rb).pos_ = 0i32 as (u32);
    (*rb).data_ = 0i32 as (*mut u8);
    (*rb).buffer_index = 0i32 as (usize);
}

unsafe extern fn BrotliEncoderInitState(
    mut s : *mut BrotliEncoderStateStruct
) {
    BrotliEncoderInitParams(
        &mut (*s).params as (*mut BrotliEncoderParams)
    );
    (*s).input_pos_ = 0i32 as (usize);
    (*s).num_commands_ = 0i32 as (usize);
    (*s).num_literals_ = 0i32 as (usize);
    (*s).last_insert_len_ = 0i32 as (usize);
    (*s).last_flush_pos_ = 0i32 as (usize);
    (*s).last_processed_pos_ = 0i32 as (usize);
    (*s).prev_byte_ = 0i32 as (u8);
    (*s).prev_byte2_ = 0i32 as (u8);
    (*s).storage_size_ = 0i32 as (usize);
    (*s).storage_ = 0i32 as (*mut u8);
    (*s).hasher_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    (*s).large_table_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut i32);
    (*s).large_table_size_ = 0i32 as (usize);
    (*s).cmd_code_numbits_ = 0i32 as (usize);
    (*s).command_buf_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
    (*s).literal_buf_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    (*s).next_out_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    (*s).available_out_ = 0i32 as (usize);
    (*s).total_out_ = 0i32 as (usize);
    (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING;
    (*s).is_last_block_emitted_ = 0i32;
    (*s).is_initialized_ = 0i32;
    RingBufferInit(&mut (*s).ringbuffer_ as (*mut RingBuffer));
    (*s).commands_ = 0i32 as (*mut Command);
    (*s).cmd_alloc_size_ = 0i32 as (usize);
    (*s).dist_cache_[0i32 as (usize)] = 4i32;
    (*s).dist_cache_[1i32 as (usize)] = 11i32;
    (*s).dist_cache_[2i32 as (usize)] = 15i32;
    (*s).dist_cache_[3i32 as (usize)] = 16i32;
    memcpy(
        (*s).saved_dist_cache_.as_mut_ptr(
        ) as (*mut ::std::os::raw::c_void),
        (*s).dist_cache_.as_mut_ptr() as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<[i32; 4]>()
    );
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderCreateInstance(
    mut
    alloc_func
    :
    unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void,
    mut
    free_func
    :
    unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
    mut opaque : *mut ::std::os::raw::c_void
) -> *mut BrotliEncoderStateStruct {
    let mut state
        : *mut BrotliEncoderStateStruct
        = 0i32 as (*mut BrotliEncoderStateStruct);
    if alloc_func == 0 && (free_func == 0) {
        state = malloc(
                    ::std::mem::size_of::<BrotliEncoderStateStruct>()
                ) as (*mut BrotliEncoderStateStruct);
    } else if alloc_func != 0 && (free_func != 0) {
        state = alloc_func(
                    opaque,
                    ::std::mem::size_of::<BrotliEncoderStateStruct>()
                ) as (*mut BrotliEncoderStateStruct);
    }
    if state == 0i32 as (*mut BrotliEncoderStateStruct) {
        return 0i32 as (*mut BrotliEncoderStateStruct);
    }
    BrotliInitMemoryManager(
        &mut (*state).memory_manager_ as (*mut MemoryManager),
        alloc_func,
        free_func,
        opaque
    );
    BrotliEncoderInitState(state);
    state
}

unsafe extern fn RingBufferFree(
    mut m : *mut MemoryManager, mut rb : *mut RingBuffer
) {
    BrotliFree(m,(*rb).data_ as (*mut ::std::os::raw::c_void));
    (*rb).data_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
}

unsafe extern fn DestroyHasher(
    mut m : *mut MemoryManager, mut handle : *mut *mut u8
) {
    if *handle == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        return;
    }
    {
        BrotliFree(m,*handle as (*mut ::std::os::raw::c_void));
        *handle = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
}

unsafe extern fn BrotliEncoderCleanupState(
    mut s : *mut BrotliEncoderStateStruct
) {
    let mut m
        : *mut MemoryManager
        = &mut (*s).memory_manager_ as (*mut MemoryManager);
    if !(0i32 == 0) {
        BrotliWipeOutMemoryManager(m);
        return;
    }
    {
        BrotliFree(m,(*s).storage_ as (*mut ::std::os::raw::c_void));
        (*s).storage_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    {
        BrotliFree(m,(*s).commands_ as (*mut ::std::os::raw::c_void));
        (*s).commands_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut Command);
    }
    RingBufferFree(m,&mut (*s).ringbuffer_ as (*mut RingBuffer));
    DestroyHasher(m,&mut (*s).hasher_ as (*mut *mut u8));
    {
        BrotliFree(m,(*s).large_table_ as (*mut ::std::os::raw::c_void));
        (*s).large_table_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut i32);
    }
    {
        BrotliFree(m,(*s).command_buf_ as (*mut ::std::os::raw::c_void));
        (*s).command_buf_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
    }
    {
        BrotliFree(m,(*s).literal_buf_ as (*mut ::std::os::raw::c_void));
        (*s).literal_buf_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderDestroyInstance(
    mut state : *mut BrotliEncoderStateStruct
) { if state.is_null() {
    } else {
        let mut m
            : *mut MemoryManager
            = &mut (*state).memory_manager_ as (*mut MemoryManager);
        let mut free_func
            : unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void)
            = (*m).free_func;
        let mut opaque : *mut ::std::os::raw::c_void = (*m).opaque;
        BrotliEncoderCleanupState(state);
        free_func(opaque,state as (*mut ::std::os::raw::c_void));
    }
}

unsafe extern fn brotli_min_int(mut a : i32, mut b : i32) -> i32 {
    if a < b { a } else { b }
}

unsafe extern fn brotli_max_int(mut a : i32, mut b : i32) -> i32 {
    if a > b { a } else { b }
}

unsafe extern fn SanitizeParams(
    mut params : *mut BrotliEncoderParams
) {
    (*params).quality = brotli_min_int(
                            11i32,
                            brotli_max_int(0i32,(*params).quality)
                        );
    if (*params).lgwin < 10i32 {
        (*params).lgwin = 10i32;
    } else if (*params).lgwin > 24i32 {
        (*params).lgwin = 24i32;
    }
}

unsafe extern fn ComputeLgBlock(
    mut params : *const BrotliEncoderParams
) -> i32 {
    let mut lgblock : i32 = (*params).lgblock;
    if (*params).quality == 0i32 || (*params).quality == 1i32 {
        lgblock = (*params).lgwin;
    } else if (*params).quality < 4i32 {
        lgblock = 14i32;
    } else if lgblock == 0i32 {
        lgblock = 16i32;
        if (*params).quality >= 9i32 && ((*params).lgwin > lgblock) {
            lgblock = brotli_min_int(18i32,(*params).lgwin);
        }
    } else {
        lgblock = brotli_min_int(24i32,brotli_max_int(16i32,lgblock));
    }
    lgblock
}

unsafe extern fn ComputeRbBits(
    mut params : *const BrotliEncoderParams
) -> i32 {
    1i32 + brotli_max_int((*params).lgwin,(*params).lgblock)
}

unsafe extern fn RingBufferSetup(
    mut params : *const BrotliEncoderParams, mut rb : *mut RingBuffer
) {
    let mut window_bits : i32 = ComputeRbBits(params);
    let mut tail_bits : i32 = (*params).lgblock;
    *(&mut (*rb).size_ as (*mut u32)) = 1u32 << window_bits;
    *(&mut (*rb).mask_ as (*mut u32)) = (1u32 << window_bits).wrapping_sub(
                                            1i32 as (u32)
                                        );
    *(&mut (*rb).tail_size_ as (*mut u32)) = 1u32 << tail_bits;
    *(&mut (*rb).total_size_ as (*mut u32)) = (*rb).size_.wrapping_add(
                                                  (*rb).tail_size_
                                              );
}

unsafe extern fn EncodeWindowBits(
    mut lgwin : i32,
    mut last_byte : *mut u8,
    mut last_byte_bits : *mut u8
) { if lgwin == 16i32 {
        *last_byte = 0i32 as (u8);
        *last_byte_bits = 1i32 as (u8);
    } else if lgwin == 17i32 {
        *last_byte = 1i32 as (u8);
        *last_byte_bits = 7i32 as (u8);
    } else if lgwin > 17i32 {
        *last_byte = (lgwin - 17i32 << 1i32 | 1i32) as (u8);
        *last_byte_bits = 4i32 as (u8);
    } else {
        *last_byte = (lgwin - 8i32 << 4i32 | 1i32) as (u8);
        *last_byte_bits = 7i32 as (u8);
    }
}

unsafe extern fn InitCommandPrefixCodes(
    mut cmd_depths : *mut u8,
    mut cmd_bits : *mut u16,
    mut cmd_code : *mut u8,
    mut cmd_code_numbits : *mut usize
) {
    static mut kDefaultCommandDepths
        : [u8; 128]
        = [   0i32 as (u8),
              4i32 as (u8),
              4i32 as (u8),
              5i32 as (u8),
              6i32 as (u8),
              6i32 as (u8),
              7i32 as (u8),
              7i32 as (u8),
              7i32 as (u8),
              7i32 as (u8),
              7i32 as (u8),
              8i32 as (u8),
              8i32 as (u8),
              8i32 as (u8),
              8i32 as (u8),
              8i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              4i32 as (u8),
              4i32 as (u8),
              4i32 as (u8),
              4i32 as (u8),
              4i32 as (u8),
              5i32 as (u8),
              5i32 as (u8),
              6i32 as (u8),
              6i32 as (u8),
              6i32 as (u8),
              6i32 as (u8),
              7i32 as (u8),
              7i32 as (u8),
              7i32 as (u8),
              7i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              0i32 as (u8),
              4i32 as (u8),
              4i32 as (u8),
              5i32 as (u8),
              5i32 as (u8),
              5i32 as (u8),
              6i32 as (u8),
              6i32 as (u8),
              7i32 as (u8),
              8i32 as (u8),
              8i32 as (u8),
              9i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              10i32 as (u8),
              5i32 as (u8),
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
              6i32 as (u8),
              6i32 as (u8),
              6i32 as (u8),
              6i32 as (u8),
              6i32 as (u8),
              6i32 as (u8),
              5i32 as (u8),
              5i32 as (u8),
              5i32 as (u8),
              5i32 as (u8),
              5i32 as (u8),
              5i32 as (u8),
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
              6i32 as (u8),
              6i32 as (u8),
              7i32 as (u8),
              7i32 as (u8),
              7i32 as (u8),
              8i32 as (u8),
              10i32 as (u8),
              12i32 as (u8),
              12i32 as (u8),
              12i32 as (u8),
              12i32 as (u8),
              12i32 as (u8),
              12i32 as (u8),
              12i32 as (u8),
              12i32 as (u8),
              12i32 as (u8),
              12i32 as (u8),
              12i32 as (u8),
              12i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8)
          ];
    static mut kDefaultCommandBits
        : [u16; 128]
        = [   0i32 as (u16),
              0i32 as (u16),
              8i32 as (u16),
              9i32 as (u16),
              3i32 as (u16),
              35i32 as (u16),
              7i32 as (u16),
              71i32 as (u16),
              39i32 as (u16),
              103i32 as (u16),
              23i32 as (u16),
              47i32 as (u16),
              175i32 as (u16),
              111i32 as (u16),
              239i32 as (u16),
              31i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              4i32 as (u16),
              12i32 as (u16),
              2i32 as (u16),
              10i32 as (u16),
              6i32 as (u16),
              13i32 as (u16),
              29i32 as (u16),
              11i32 as (u16),
              43i32 as (u16),
              27i32 as (u16),
              59i32 as (u16),
              87i32 as (u16),
              55i32 as (u16),
              15i32 as (u16),
              79i32 as (u16),
              319i32 as (u16),
              831i32 as (u16),
              191i32 as (u16),
              703i32 as (u16),
              447i32 as (u16),
              959i32 as (u16),
              0i32 as (u16),
              14i32 as (u16),
              1i32 as (u16),
              25i32 as (u16),
              5i32 as (u16),
              21i32 as (u16),
              19i32 as (u16),
              51i32 as (u16),
              119i32 as (u16),
              159i32 as (u16),
              95i32 as (u16),
              223i32 as (u16),
              479i32 as (u16),
              991i32 as (u16),
              63i32 as (u16),
              575i32 as (u16),
              127i32 as (u16),
              639i32 as (u16),
              383i32 as (u16),
              895i32 as (u16),
              255i32 as (u16),
              767i32 as (u16),
              511i32 as (u16),
              1023i32 as (u16),
              14i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              27i32 as (u16),
              59i32 as (u16),
              7i32 as (u16),
              39i32 as (u16),
              23i32 as (u16),
              55i32 as (u16),
              30i32 as (u16),
              1i32 as (u16),
              17i32 as (u16),
              9i32 as (u16),
              25i32 as (u16),
              5i32 as (u16),
              0i32 as (u16),
              8i32 as (u16),
              4i32 as (u16),
              12i32 as (u16),
              2i32 as (u16),
              10i32 as (u16),
              6i32 as (u16),
              21i32 as (u16),
              13i32 as (u16),
              29i32 as (u16),
              3i32 as (u16),
              19i32 as (u16),
              11i32 as (u16),
              15i32 as (u16),
              47i32 as (u16),
              31i32 as (u16),
              95i32 as (u16),
              63i32 as (u16),
              127i32 as (u16),
              255i32 as (u16),
              767i32 as (u16),
              2815i32 as (u16),
              1791i32 as (u16),
              3839i32 as (u16),
              511i32 as (u16),
              2559i32 as (u16),
              1535i32 as (u16),
              3583i32 as (u16),
              1023i32 as (u16),
              3071i32 as (u16),
              2047i32 as (u16),
              4095i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16)
          ];
    static mut kDefaultCommandCode
        : [u8; 57]
        = [   0xffi32 as (u8),
              0x77i32 as (u8),
              0xd5i32 as (u8),
              0xbfi32 as (u8),
              0xe7i32 as (u8),
              0xdei32 as (u8),
              0xeai32 as (u8),
              0x9ei32 as (u8),
              0x51i32 as (u8),
              0x5di32 as (u8),
              0xdei32 as (u8),
              0xc6i32 as (u8),
              0x70i32 as (u8),
              0x57i32 as (u8),
              0xbci32 as (u8),
              0x58i32 as (u8),
              0x58i32 as (u8),
              0x58i32 as (u8),
              0xd8i32 as (u8),
              0xd8i32 as (u8),
              0x58i32 as (u8),
              0xd5i32 as (u8),
              0xcbi32 as (u8),
              0x8ci32 as (u8),
              0xeai32 as (u8),
              0xe0i32 as (u8),
              0xc3i32 as (u8),
              0x87i32 as (u8),
              0x1fi32 as (u8),
              0x83i32 as (u8),
              0xc1i32 as (u8),
              0x60i32 as (u8),
              0x1ci32 as (u8),
              0x67i32 as (u8),
              0xb2i32 as (u8),
              0xaai32 as (u8),
              0x6i32 as (u8),
              0x83i32 as (u8),
              0xc1i32 as (u8),
              0x60i32 as (u8),
              0x30i32 as (u8),
              0x18i32 as (u8),
              0xcci32 as (u8),
              0xa1i32 as (u8),
              0xcei32 as (u8),
              0x88i32 as (u8),
              0x54i32 as (u8),
              0x94i32 as (u8),
              0x46i32 as (u8),
              0xe1i32 as (u8),
              0xb0i32 as (u8),
              0xd0i32 as (u8),
              0x4ei32 as (u8),
              0xb2i32 as (u8),
              0xf7i32 as (u8),
              0x4i32 as (u8),
              0x0i32 as (u8)
          ];
    static kDefaultCommandCodeNumBits : usize = 448i32 as (usize);
    memcpy(
        cmd_depths as (*mut ::std::os::raw::c_void),
        kDefaultCommandDepths.as_ptr() as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<[u8; 128]>()
    );
    memcpy(
        cmd_bits as (*mut ::std::os::raw::c_void),
        kDefaultCommandBits.as_ptr() as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<[u16; 128]>()
    );
    memcpy(
        cmd_code as (*mut ::std::os::raw::c_void),
        kDefaultCommandCode.as_ptr() as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<[u8; 57]>()
    );
    *cmd_code_numbits = kDefaultCommandCodeNumBits;
}

unsafe extern fn EnsureInitialized(
    mut s : *mut BrotliEncoderStateStruct
) -> i32 {
    if !(0i32 == 0) {
        return 0i32;
    }
    if (*s).is_initialized_ != 0 {
        return 1i32;
    }
    SanitizeParams(&mut (*s).params as (*mut BrotliEncoderParams));
    (*s).params.lgblock = ComputeLgBlock(
                              &mut (*s).params as (*mut BrotliEncoderParams) as (*const BrotliEncoderParams)
                          );
    (*s).remaining_metadata_bytes_ = !(0i32 as (u32));
    RingBufferSetup(
        &mut (*s).params as (*mut BrotliEncoderParams) as (*const BrotliEncoderParams),
        &mut (*s).ringbuffer_ as (*mut RingBuffer)
    );
    {
        let mut lgwin : i32 = (*s).params.lgwin;
        if (*s).params.quality == 0i32 || (*s).params.quality == 1i32 {
            lgwin = brotli_max_int(lgwin,18i32);
        }
        EncodeWindowBits(
            lgwin,
            &mut (*s).last_byte_ as (*mut u8),
            &mut (*s).last_byte_bits_ as (*mut u8)
        );
    }
    if (*s).params.quality == 0i32 {
        InitCommandPrefixCodes(
            (*s).cmd_depths_.as_mut_ptr(),
            (*s).cmd_bits_.as_mut_ptr(),
            (*s).cmd_code_.as_mut_ptr(),
            &mut (*s).cmd_code_numbits_ as (*mut usize)
        );
    }
    (*s).is_initialized_ = 1i32;
    1i32
}

unsafe extern fn RingBufferInitBuffer(
    mut m : *mut MemoryManager, buflen : u32, mut rb : *mut RingBuffer
) {
    static kSlackForEightByteHashingEverywhere
        : usize
        = 7i32 as (usize);
    let mut new_data
        : *mut u8
        = if ((2i32 as (u32)).wrapping_add(
                  buflen
              ) as (usize)).wrapping_add(
                 kSlackForEightByteHashingEverywhere
             ) != 0 {
              BrotliAllocate(
                  m,
                  ((2i32 as (u32)).wrapping_add(buflen) as (usize)).wrapping_add(
                      kSlackForEightByteHashingEverywhere
                  ).wrapping_mul(
                      ::std::mem::size_of::<u8>()
                  )
              ) as (*mut u8)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
          };
    let mut i : usize;
    if !(0i32 == 0) {
        return;
    }
    if !(*rb).data_.is_null() {
        memcpy(
            new_data as (*mut ::std::os::raw::c_void),
            (*rb).data_ as (*const ::std::os::raw::c_void),
            ((2i32 as (u32)).wrapping_add(
                 (*rb).cur_size_
             ) as (usize)).wrapping_add(
                kSlackForEightByteHashingEverywhere
            )
        );
        {
            BrotliFree(m,(*rb).data_ as (*mut ::std::os::raw::c_void));
            (*rb).data_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
    }
    (*rb).data_ = new_data;
    (*rb).cur_size_ = buflen;
    (*rb).buffer_index = 2i32 as (usize);
    *(*rb).data_.offset(
         (*rb).buffer_index.wrapping_sub(2i32 as (usize)) as (isize)
     ) = {
             let _rhs = 0i32;
             let _lhs
                 = &mut *(*rb).data_.offset(
                             (*rb).buffer_index.wrapping_sub(1i32 as (usize)) as (isize)
                         );
             *_lhs = _rhs as (u8);
             *_lhs
         };
    i = 0i32 as (usize);
    while i < kSlackForEightByteHashingEverywhere {
        {
            *(*rb).data_.offset(
                 (*rb).buffer_index.wrapping_add(
                     (*rb).cur_size_ as (usize)
                 ).wrapping_add(
                     i
                 ) as (isize)
             ) = 0i32 as (u8);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
}

unsafe extern fn RingBufferWriteTail(
    mut bytes : *const u8, mut n : usize, mut rb : *mut RingBuffer
) {
    let masked_pos : usize = ((*rb).pos_ & (*rb).mask_) as (usize);
    if masked_pos < (*rb).tail_size_ as (usize) {
        let p : usize = ((*rb).size_ as (usize)).wrapping_add(masked_pos);
        memcpy(
            &mut *(*rb).data_.offset(
                      (*rb).buffer_index.wrapping_add(p) as (isize)
                  ) as (*mut u8) as (*mut ::std::os::raw::c_void),
            bytes as (*const ::std::os::raw::c_void),
            brotli_min_size_t(
                n,
                ((*rb).tail_size_ as (usize)).wrapping_sub(masked_pos)
            )
        );
    }
}

unsafe extern fn RingBufferWrite(
    mut m : *mut MemoryManager,
    mut bytes : *const u8,
    mut n : usize,
    mut rb : *mut RingBuffer
) {
    if (*rb).pos_ == 0i32 as (u32) && (n < (*rb).tail_size_ as (usize)) {
        (*rb).pos_ = n as (u32);
        RingBufferInitBuffer(m,(*rb).pos_,rb);
        if !(0i32 == 0) {
            return;
        }
        memcpy(
            &mut *(*rb).data_.offset(
                      (*rb).buffer_index as (isize)
                  ) as (*mut u8) as (*mut ::std::os::raw::c_void),
            bytes as (*const ::std::os::raw::c_void),
            n
        );
        return;
    }
    if (*rb).cur_size_ < (*rb).total_size_ {
        RingBufferInitBuffer(m,(*rb).total_size_,rb);
        if !(0i32 == 0) {
            return;
        }
        *(*rb).data_.offset(
             (*rb).buffer_index.wrapping_add(
                 (*rb).size_ as (usize)
             ).wrapping_sub(
                 2i32 as (usize)
             ) as (isize)
         ) = 0i32 as (u8);
        *(*rb).data_.offset(
             (*rb).buffer_index.wrapping_add(
                 (*rb).size_ as (usize)
             ).wrapping_sub(
                 1i32 as (usize)
             ) as (isize)
         ) = 0i32 as (u8);
    }
    {
        let masked_pos : usize = ((*rb).pos_ & (*rb).mask_) as (usize);
        RingBufferWriteTail(bytes,n,rb);
        if masked_pos.wrapping_add(n) <= (*rb).size_ as (usize) {
            memcpy(
                &mut *(*rb).data_.offset(
                          (*rb).buffer_index.wrapping_add(masked_pos) as (isize)
                      ) as (*mut u8) as (*mut ::std::os::raw::c_void),
                bytes as (*const ::std::os::raw::c_void),
                n
            );
        } else {
            memcpy(
                &mut *(*rb).data_.offset(
                          (*rb).buffer_index.wrapping_add(masked_pos) as (isize)
                      ) as (*mut u8) as (*mut ::std::os::raw::c_void),
                bytes as (*const ::std::os::raw::c_void),
                brotli_min_size_t(
                    n,
                    ((*rb).total_size_ as (usize)).wrapping_sub(masked_pos)
                )
            );
            memcpy(
                &mut *(*rb).data_.offset(
                          (*rb).buffer_index.wrapping_add(0i32 as (usize)) as (isize)
                      ) as (*mut u8) as (*mut ::std::os::raw::c_void),
                bytes.offset(
                    ((*rb).size_ as (usize)).wrapping_sub(masked_pos) as (isize)
                ) as (*const ::std::os::raw::c_void),
                n.wrapping_sub(((*rb).size_ as (usize)).wrapping_sub(masked_pos))
            );
        }
    }
    *(*rb).data_.offset(
         (*rb).buffer_index.wrapping_sub(2i32 as (usize)) as (isize)
     ) = *(*rb).data_.offset(
              (*rb).buffer_index.wrapping_add(
                  (*rb).size_ as (usize)
              ).wrapping_sub(
                  2i32 as (usize)
              ) as (isize)
          );
    *(*rb).data_.offset(
         (*rb).buffer_index.wrapping_sub(1i32 as (usize)) as (isize)
     ) = *(*rb).data_.offset(
              (*rb).buffer_index.wrapping_add(
                  (*rb).size_ as (usize)
              ).wrapping_sub(
                  1i32 as (usize)
              ) as (isize)
          );
    (*rb).pos_ = (*rb).pos_.wrapping_add(n as (u32));
    if (*rb).pos_ > 1u32 << 30i32 {
        (*rb).pos_ = (*rb).pos_ & (1u32 << 30i32).wrapping_sub(
                                      1i32 as (u32)
                                  ) | 1u32 << 30i32;
    }
}

unsafe extern fn CopyInputToRingBuffer(
    mut s : *mut BrotliEncoderStateStruct,
    input_size : usize,
    mut input_buffer : *const u8
) {
    let mut ringbuffer_
        : *mut RingBuffer
        = &mut (*s).ringbuffer_ as (*mut RingBuffer);
    let mut m
        : *mut MemoryManager
        = &mut (*s).memory_manager_ as (*mut MemoryManager);
    if EnsureInitialized(s) == 0 {
        return;
    }
    RingBufferWrite(m,input_buffer,input_size,ringbuffer_);
    if !(0i32 == 0) {
        return;
    }
    (*s).input_pos_ = (*s).input_pos_.wrapping_add(input_size);
    if (*ringbuffer_).pos_ <= (*ringbuffer_).mask_ {
        memset(
            &mut *(*ringbuffer_).data_.offset(
                      (*ringbuffer_).buffer_index.wrapping_add(
                          (*ringbuffer_).pos_ as (usize)
                      ) as (isize)
                  ) as (*mut u8) as (*mut ::std::os::raw::c_void),
            0i32,
            7i32 as (usize)
        );
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Struct4 {
    pub params : BrotliHasherParams,
    pub is_prepared_ : i32,
    pub dict_num_lookups : usize,
    pub dict_num_matches : usize,
}

unsafe extern fn ChooseHasher(
    mut params : *const BrotliEncoderParams,
    mut hparams : *mut BrotliHasherParams
) { if (*params).quality > 9i32 {
        (*hparams).type_ = 10i32;
    } else if (*params).quality == 4i32 && ((*params).size_hint >= (1i32 << 20i32) as (usize)) {
        (*hparams).type_ = 54i32;
    } else if (*params).quality < 5i32 {
        (*hparams).type_ = (*params).quality;
    } else if (*params).lgwin <= 16i32 {
        (*hparams).type_ = if (*params).quality < 7i32 {
                               40i32
                           } else if (*params).quality < 9i32 {
                               41i32
                           } else {
                               42i32
                           };
    } else if (*params).size_hint >= (1i32 << 20i32) as (usize) && ((*params).lgwin >= 19i32) {
        (*hparams).type_ = 6i32;
        (*hparams).block_bits = (*params).quality - 1i32;
        (*hparams).bucket_bits = 15i32;
        (*hparams).hash_len = 5i32;
        (*hparams).num_last_distances_to_check = if (*params).quality < 7i32 {
                                                     4i32
                                                 } else if (*params).quality < 9i32 {
                                                     10i32
                                                 } else {
                                                     16i32
                                                 };
    } else {
        (*hparams).type_ = 5i32;
        (*hparams).block_bits = (*params).quality - 1i32;
        (*hparams).bucket_bits = if (*params).quality < 7i32 {
                                     14i32
                                 } else {
                                     15i32
                                 };
        (*hparams).num_last_distances_to_check = if (*params).quality < 7i32 {
                                                     4i32
                                                 } else if (*params).quality < 9i32 {
                                                     10i32
                                                 } else {
                                                     16i32
                                                 };
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H2 {
    pub buckets_ : [u32; 65537],
}

unsafe extern fn HashMemAllocInBytesH2(
    mut params : *const BrotliEncoderParams,
    mut one_shot : i32,
    mut input_size : usize
) -> usize {
    params;
    one_shot;
    input_size;
    ::std::mem::size_of::<H2>()
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H3 {
    pub buckets_ : [u32; 65538],
}

unsafe extern fn HashMemAllocInBytesH3(
    mut params : *const BrotliEncoderParams,
    mut one_shot : i32,
    mut input_size : usize
) -> usize {
    params;
    one_shot;
    input_size;
    ::std::mem::size_of::<H3>()
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H4 {
    pub buckets_ : [u32; 131076],
}

unsafe extern fn HashMemAllocInBytesH4(
    mut params : *const BrotliEncoderParams,
    mut one_shot : i32,
    mut input_size : usize
) -> usize {
    params;
    one_shot;
    input_size;
    ::std::mem::size_of::<H4>()
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H5 {
    pub bucket_size_ : usize,
    pub block_size_ : usize,
    pub hash_shift_ : i32,
    pub block_mask_ : u32,
}

unsafe extern fn HashMemAllocInBytesH5(
    mut params : *const BrotliEncoderParams,
    mut one_shot : i32,
    mut input_size : usize
) -> usize {
    let mut bucket_size
        : usize
        = 1i32 as (usize) << (*params).hasher.bucket_bits;
    let mut block_size
        : usize
        = 1i32 as (usize) << (*params).hasher.block_bits;
    one_shot;
    input_size;
    ::std::mem::size_of::<H5>().wrapping_add(
        bucket_size.wrapping_mul(
            (2i32 as (usize)).wrapping_add(
                (4i32 as (usize)).wrapping_mul(block_size)
            )
        )
    )
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H6 {
    pub bucket_size_ : usize,
    pub block_size_ : usize,
    pub hash_shift_ : i32,
    pub hash_mask_ : usize,
    pub block_mask_ : u32,
}

unsafe extern fn HashMemAllocInBytesH6(
    mut params : *const BrotliEncoderParams,
    mut one_shot : i32,
    mut input_size : usize
) -> usize {
    let mut bucket_size
        : usize
        = 1i32 as (usize) << (*params).hasher.bucket_bits;
    let mut block_size
        : usize
        = 1i32 as (usize) << (*params).hasher.block_bits;
    one_shot;
    input_size;
    ::std::mem::size_of::<H6>().wrapping_add(
        bucket_size.wrapping_mul(
            (2i32 as (usize)).wrapping_add(
                (4i32 as (usize)).wrapping_mul(block_size)
            )
        )
    )
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SlotH40 {
    pub delta : u16,
    pub next : u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BankH40 {
    pub slots : [SlotH40; 65536],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H40 {
    pub addr : [u32; 32768],
    pub head : [u16; 32768],
    pub tiny_hash : [u8; 65536],
    pub banks : [BankH40; 1],
    pub free_slot_idx : [u16; 1],
    pub max_hops : usize,
}

unsafe extern fn HashMemAllocInBytesH40(
    mut params : *const BrotliEncoderParams,
    mut one_shot : i32,
    mut input_size : usize
) -> usize {
    params;
    one_shot;
    input_size;
    ::std::mem::size_of::<H40>()
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SlotH41 {
    pub delta : u16,
    pub next : u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BankH41 {
    pub slots : [SlotH41; 65536],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H41 {
    pub addr : [u32; 32768],
    pub head : [u16; 32768],
    pub tiny_hash : [u8; 65536],
    pub banks : [BankH41; 1],
    pub free_slot_idx : [u16; 1],
    pub max_hops : usize,
}

unsafe extern fn HashMemAllocInBytesH41(
    mut params : *const BrotliEncoderParams,
    mut one_shot : i32,
    mut input_size : usize
) -> usize {
    params;
    one_shot;
    input_size;
    ::std::mem::size_of::<H41>()
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SlotH42 {
    pub delta : u16,
    pub next : u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BankH42 {
    pub slots : [SlotH42; 512],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H42 {
    pub addr : [u32; 32768],
    pub head : [u16; 32768],
    pub tiny_hash : [u8; 65536],
    pub banks : [BankH42; 512],
    pub free_slot_idx : [u16; 512],
    pub max_hops : usize,
}

unsafe extern fn HashMemAllocInBytesH42(
    mut params : *const BrotliEncoderParams,
    mut one_shot : i32,
    mut input_size : usize
) -> usize {
    params;
    one_shot;
    input_size;
    ::std::mem::size_of::<H42>()
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H54 {
    pub buckets_ : [u32; 1048580],
}

unsafe extern fn HashMemAllocInBytesH54(
    mut params : *const BrotliEncoderParams,
    mut one_shot : i32,
    mut input_size : usize
) -> usize {
    params;
    one_shot;
    input_size;
    ::std::mem::size_of::<H54>()
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H10 {
    pub window_mask_ : usize,
    pub buckets_ : [u32; 131072],
    pub invalid_pos_ : u32,
}

unsafe extern fn HashMemAllocInBytesH10(
    mut params : *const BrotliEncoderParams,
    mut one_shot : i32,
    mut input_size : usize
) -> usize {
    let mut num_nodes : usize = 1i32 as (usize) << (*params).lgwin;
    if one_shot != 0 && (input_size < num_nodes) {
        num_nodes = input_size;
    }
    ::std::mem::size_of::<H10>().wrapping_add(
        (2i32 as (usize)).wrapping_mul(
            ::std::mem::size_of::<u32>()
        ).wrapping_mul(
            num_nodes
        )
    )
}

unsafe extern fn HasherSize(
    mut params : *const BrotliEncoderParams,
    mut one_shot : i32,
    input_size : usize
) -> usize {
    let mut result : usize = ::std::mem::size_of::<Struct4>();
    let mut hashtype : i32 = (*params).hasher.type_;
    if hashtype == 2i32 {
        result = result.wrapping_add(
                     HashMemAllocInBytesH2(params,one_shot,input_size)
                 );
    }
    if hashtype == 3i32 {
        result = result.wrapping_add(
                     HashMemAllocInBytesH3(params,one_shot,input_size)
                 );
    }
    if hashtype == 4i32 {
        result = result.wrapping_add(
                     HashMemAllocInBytesH4(params,one_shot,input_size)
                 );
    }
    if hashtype == 5i32 {
        result = result.wrapping_add(
                     HashMemAllocInBytesH5(params,one_shot,input_size)
                 );
    }
    if hashtype == 6i32 {
        result = result.wrapping_add(
                     HashMemAllocInBytesH6(params,one_shot,input_size)
                 );
    }
    if hashtype == 40i32 {
        result = result.wrapping_add(
                     HashMemAllocInBytesH40(params,one_shot,input_size)
                 );
    }
    if hashtype == 41i32 {
        result = result.wrapping_add(
                     HashMemAllocInBytesH41(params,one_shot,input_size)
                 );
    }
    if hashtype == 42i32 {
        result = result.wrapping_add(
                     HashMemAllocInBytesH42(params,one_shot,input_size)
                 );
    }
    if hashtype == 54i32 {
        result = result.wrapping_add(
                     HashMemAllocInBytesH54(params,one_shot,input_size)
                 );
    }
    if hashtype == 10i32 {
        result = result.wrapping_add(
                     HashMemAllocInBytesH10(params,one_shot,input_size)
                 );
    }
    result
}

unsafe extern fn GetHasherCommon(
    mut handle : *mut u8
) -> *mut Struct4 {
    handle as (*mut Struct4)
}

unsafe extern fn InitializeH2(
    mut handle : *mut u8, mut params : *const BrotliEncoderParams
) {
    handle;
    params;
}

unsafe extern fn InitializeH3(
    mut handle : *mut u8, mut params : *const BrotliEncoderParams
) {
    handle;
    params;
}

unsafe extern fn InitializeH4(
    mut handle : *mut u8, mut params : *const BrotliEncoderParams
) {
    handle;
    params;
}

unsafe extern fn SelfH5(mut handle : *mut u8) -> *mut H5 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct4) as (*mut H5)
}

unsafe extern fn InitializeH5(
    mut handle : *mut u8, mut params : *const BrotliEncoderParams
) {
    let mut common : *mut Struct4 = GetHasherCommon(handle);
    let mut self : *mut H5 = SelfH5(handle);
    params;
    (*self).hash_shift_ = 32i32 - (*common).params.bucket_bits;
    (*self).bucket_size_ = 1i32 as (usize) << (*common).params.bucket_bits;
    (*self).block_size_ = 1i32 as (usize) << (*common).params.block_bits;
    (*self).block_mask_ = (*self).block_size_.wrapping_sub(
                              1i32 as (usize)
                          ) as (u32);
}

unsafe extern fn SelfH6(mut handle : *mut u8) -> *mut H6 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct4) as (*mut H6)
}

unsafe extern fn InitializeH6(
    mut handle : *mut u8, mut params : *const BrotliEncoderParams
) {
    let mut common : *mut Struct4 = GetHasherCommon(handle);
    let mut self : *mut H6 = SelfH6(handle);
    params;
    (*self).hash_shift_ = 64i32 - (*common).params.bucket_bits;
    (*self).hash_mask_ = !(0u32 as (usize)) >> 64i32 - 8i32 * (*common).params.hash_len;
    (*self).bucket_size_ = 1i32 as (usize) << (*common).params.bucket_bits;
    (*self).block_size_ = 1i32 as (usize) << (*common).params.block_bits;
    (*self).block_mask_ = (*self).block_size_.wrapping_sub(
                              1i32 as (usize)
                          ) as (u32);
}

unsafe extern fn SelfH40(mut handle : *mut u8) -> *mut H40 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct4) as (*mut H40)
}

unsafe extern fn InitializeH40(
    mut handle : *mut u8, mut params : *const BrotliEncoderParams
) {
    (*SelfH40(handle)).max_hops = (if (*params).quality > 6i32 {
                                       7u32
                                   } else {
                                       8u32
                                   } << (*params).quality - 4i32) as (usize);
}

unsafe extern fn SelfH41(mut handle : *mut u8) -> *mut H41 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct4) as (*mut H41)
}

unsafe extern fn InitializeH41(
    mut handle : *mut u8, mut params : *const BrotliEncoderParams
) {
    (*SelfH41(handle)).max_hops = (if (*params).quality > 6i32 {
                                       7u32
                                   } else {
                                       8u32
                                   } << (*params).quality - 4i32) as (usize);
}

unsafe extern fn SelfH42(mut handle : *mut u8) -> *mut H42 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct4) as (*mut H42)
}

unsafe extern fn InitializeH42(
    mut handle : *mut u8, mut params : *const BrotliEncoderParams
) {
    (*SelfH42(handle)).max_hops = (if (*params).quality > 6i32 {
                                       7u32
                                   } else {
                                       8u32
                                   } << (*params).quality - 4i32) as (usize);
}

unsafe extern fn InitializeH54(
    mut handle : *mut u8, mut params : *const BrotliEncoderParams
) {
    handle;
    params;
}

unsafe extern fn SelfH10(mut handle : *mut u8) -> *mut H10 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct4) as (*mut H10)
}

unsafe extern fn InitializeH10(
    mut handle : *mut u8, mut params : *const BrotliEncoderParams
) {
    let mut self : *mut H10 = SelfH10(handle);
    (*self).window_mask_ = (1u32 << (*params).lgwin).wrapping_sub(
                               1u32
                           ) as (usize);
    (*self).invalid_pos_ = (0i32 as (usize)).wrapping_sub(
                               (*self).window_mask_
                           ) as (u32);
}

unsafe extern fn HasherReset(mut handle : *mut u8) {
    if handle == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        return;
    }
    (*GetHasherCommon(handle)).is_prepared_ = 0i32;
}

unsafe extern fn SelfH2(mut handle : *mut u8) -> *mut H2 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct4) as (*mut H2)
}

unsafe extern fn BROTLI_UNALIGNED_LOAD64(
    mut p : *const ::std::os::raw::c_void
) -> usize {
    let mut t : usize;
    memcpy(
        &mut t as (*mut usize) as (*mut ::std::os::raw::c_void),
        p,
        ::std::mem::size_of::<usize>()
    );
    t
}

unsafe extern fn HashBytesH2(mut data : *const u8) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               data as (*const ::std::os::raw::c_void)
           ) << 64i32 - 8i32 * 5i32).wrapping_mul(
              kHashMul64
          );
    (h >> 64i32 - 16i32) as (u32)
}

unsafe extern fn PrepareH2(
    mut handle : *mut u8,
    mut one_shot : i32,
    mut input_size : usize,
    mut data : *const u8
) {
    let mut self : *mut H2 = SelfH2(handle);
    let mut partial_prepare_threshold
        : usize
        = (4i32 << 16i32 >> 7i32) as (usize);
    if one_shot != 0 && (input_size <= partial_prepare_threshold) {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < input_size {
            {
                let key
                    : u32
                    = HashBytesH2(&*data.offset(i as (isize)) as (*const u8));
                memset(
                    &mut (*self).buckets_[
                             key as (usize)
                         ] as (*mut u32) as (*mut ::std::os::raw::c_void),
                    0i32,
                    (1i32 as (usize)).wrapping_mul(::std::mem::size_of::<u32>())
                );
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        memset(
            &mut (*self).buckets_[
                     0i32 as (usize)
                 ] as (*mut u32) as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<[u32; 65537]>()
        );
    }
}

unsafe extern fn SelfH3(mut handle : *mut u8) -> *mut H3 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct4) as (*mut H3)
}

unsafe extern fn HashBytesH3(mut data : *const u8) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               data as (*const ::std::os::raw::c_void)
           ) << 64i32 - 8i32 * 5i32).wrapping_mul(
              kHashMul64
          );
    (h >> 64i32 - 16i32) as (u32)
}

unsafe extern fn PrepareH3(
    mut handle : *mut u8,
    mut one_shot : i32,
    mut input_size : usize,
    mut data : *const u8
) {
    let mut self : *mut H3 = SelfH3(handle);
    let mut partial_prepare_threshold
        : usize
        = (4i32 << 16i32 >> 7i32) as (usize);
    if one_shot != 0 && (input_size <= partial_prepare_threshold) {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < input_size {
            {
                let key
                    : u32
                    = HashBytesH3(&*data.offset(i as (isize)) as (*const u8));
                memset(
                    &mut (*self).buckets_[
                             key as (usize)
                         ] as (*mut u32) as (*mut ::std::os::raw::c_void),
                    0i32,
                    (2i32 as (usize)).wrapping_mul(::std::mem::size_of::<u32>())
                );
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        memset(
            &mut (*self).buckets_[
                     0i32 as (usize)
                 ] as (*mut u32) as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<[u32; 65538]>()
        );
    }
}

unsafe extern fn SelfH4(mut handle : *mut u8) -> *mut H4 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct4) as (*mut H4)
}

unsafe extern fn HashBytesH4(mut data : *const u8) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               data as (*const ::std::os::raw::c_void)
           ) << 64i32 - 8i32 * 5i32).wrapping_mul(
              kHashMul64
          );
    (h >> 64i32 - 17i32) as (u32)
}

unsafe extern fn PrepareH4(
    mut handle : *mut u8,
    mut one_shot : i32,
    mut input_size : usize,
    mut data : *const u8
) {
    let mut self : *mut H4 = SelfH4(handle);
    let mut partial_prepare_threshold
        : usize
        = (4i32 << 17i32 >> 7i32) as (usize);
    if one_shot != 0 && (input_size <= partial_prepare_threshold) {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < input_size {
            {
                let key
                    : u32
                    = HashBytesH4(&*data.offset(i as (isize)) as (*const u8));
                memset(
                    &mut (*self).buckets_[
                             key as (usize)
                         ] as (*mut u32) as (*mut ::std::os::raw::c_void),
                    0i32,
                    (4i32 as (usize)).wrapping_mul(::std::mem::size_of::<u32>())
                );
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        memset(
            &mut (*self).buckets_[
                     0i32 as (usize)
                 ] as (*mut u32) as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<[u32; 131076]>()
        );
    }
}

unsafe extern fn NumH5(mut self : *mut H5) -> *mut u16 {
    &mut *self.offset(1i32 as (isize)) as (*mut H5) as (*mut u16)
}

unsafe extern fn BROTLI_UNALIGNED_LOAD32(
    mut p : *const ::std::os::raw::c_void
) -> u32 {
    let mut t : u32;
    memcpy(
        &mut t as (*mut u32) as (*mut ::std::os::raw::c_void),
        p,
        ::std::mem::size_of::<u32>()
    );
    t
}

unsafe extern fn HashBytesH5(
    mut data : *const u8, shift : i32
) -> u32 {
    let mut h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data as (*const ::std::os::raw::c_void)
          ).wrapping_mul(
              kHashMul32
          );
    h >> shift
}

unsafe extern fn PrepareH5(
    mut handle : *mut u8,
    mut one_shot : i32,
    mut input_size : usize,
    mut data : *const u8
) {
    let mut self : *mut H5 = SelfH5(handle);
    let mut num : *mut u16 = NumH5(self);
    let mut partial_prepare_threshold
        : usize
        = (*self).bucket_size_ >> 6i32;
    if one_shot != 0 && (input_size <= partial_prepare_threshold) {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < input_size {
            {
                let key
                    : u32
                    = HashBytesH5(
                          &*data.offset(i as (isize)) as (*const u8),
                          (*self).hash_shift_
                      );
                *num.offset(key as (isize)) = 0i32 as (u16);
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        memset(
            num as (*mut ::std::os::raw::c_void),
            0i32,
            (*self).bucket_size_.wrapping_mul(::std::mem::size_of::<u16>())
        );
    }
}

unsafe extern fn NumH6(mut self : *mut H6) -> *mut u16 {
    &mut *self.offset(1i32 as (isize)) as (*mut H6) as (*mut u16)
}

unsafe extern fn HashBytesH6(
    mut data : *const u8, mask : usize, shift : i32
) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               data as (*const ::std::os::raw::c_void)
           ) & mask).wrapping_mul(
              kHashMul64Long
          );
    (h >> shift) as (u32)
}

unsafe extern fn PrepareH6(
    mut handle : *mut u8,
    mut one_shot : i32,
    mut input_size : usize,
    mut data : *const u8
) {
    let mut self : *mut H6 = SelfH6(handle);
    let mut num : *mut u16 = NumH6(self);
    let mut partial_prepare_threshold
        : usize
        = (*self).bucket_size_ >> 6i32;
    if one_shot != 0 && (input_size <= partial_prepare_threshold) {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < input_size {
            {
                let key
                    : u32
                    = HashBytesH6(
                          &*data.offset(i as (isize)) as (*const u8),
                          (*self).hash_mask_,
                          (*self).hash_shift_
                      );
                *num.offset(key as (isize)) = 0i32 as (u16);
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        memset(
            num as (*mut ::std::os::raw::c_void),
            0i32,
            (*self).bucket_size_.wrapping_mul(::std::mem::size_of::<u16>())
        );
    }
}

unsafe extern fn HashBytesH40(mut data : *const u8) -> usize {
    let h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data as (*const ::std::os::raw::c_void)
          ).wrapping_mul(
              kHashMul32
          );
    (h >> 32i32 - 15i32) as (usize)
}

unsafe extern fn PrepareH40(
    mut handle : *mut u8,
    mut one_shot : i32,
    mut input_size : usize,
    mut data : *const u8
) {
    let mut self : *mut H40 = SelfH40(handle);
    let mut partial_prepare_threshold
        : usize
        = (32768i32 >> 6i32) as (usize);
    if one_shot != 0 && (input_size <= partial_prepare_threshold) {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < input_size {
            {
                let mut bucket
                    : usize
                    = HashBytesH40(&*data.offset(i as (isize)) as (*const u8));
                (*self).addr[bucket] = 0xccccccccu32;
                (*self).head[bucket] = 0xcccci32 as (u16);
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        memset(
            (*self).addr.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            0xcci32,
            ::std::mem::size_of::<[u32; 32768]>()
        );
        memset(
            (*self).head.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<[u16; 32768]>()
        );
    }
    memset(
        (*self).tiny_hash.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u8; 65536]>()
    );
    memset(
        (*self).free_slot_idx.as_mut_ptr(
        ) as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u16; 1]>()
    );
}

unsafe extern fn HashBytesH41(mut data : *const u8) -> usize {
    let h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data as (*const ::std::os::raw::c_void)
          ).wrapping_mul(
              kHashMul32
          );
    (h >> 32i32 - 15i32) as (usize)
}

unsafe extern fn PrepareH41(
    mut handle : *mut u8,
    mut one_shot : i32,
    mut input_size : usize,
    mut data : *const u8
) {
    let mut self : *mut H41 = SelfH41(handle);
    let mut partial_prepare_threshold
        : usize
        = (32768i32 >> 6i32) as (usize);
    if one_shot != 0 && (input_size <= partial_prepare_threshold) {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < input_size {
            {
                let mut bucket
                    : usize
                    = HashBytesH41(&*data.offset(i as (isize)) as (*const u8));
                (*self).addr[bucket] = 0xccccccccu32;
                (*self).head[bucket] = 0xcccci32 as (u16);
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        memset(
            (*self).addr.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            0xcci32,
            ::std::mem::size_of::<[u32; 32768]>()
        );
        memset(
            (*self).head.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<[u16; 32768]>()
        );
    }
    memset(
        (*self).tiny_hash.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u8; 65536]>()
    );
    memset(
        (*self).free_slot_idx.as_mut_ptr(
        ) as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u16; 1]>()
    );
}

unsafe extern fn HashBytesH42(mut data : *const u8) -> usize {
    let h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data as (*const ::std::os::raw::c_void)
          ).wrapping_mul(
              kHashMul32
          );
    (h >> 32i32 - 15i32) as (usize)
}

unsafe extern fn PrepareH42(
    mut handle : *mut u8,
    mut one_shot : i32,
    mut input_size : usize,
    mut data : *const u8
) {
    let mut self : *mut H42 = SelfH42(handle);
    let mut partial_prepare_threshold
        : usize
        = (32768i32 >> 6i32) as (usize);
    if one_shot != 0 && (input_size <= partial_prepare_threshold) {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < input_size {
            {
                let mut bucket
                    : usize
                    = HashBytesH42(&*data.offset(i as (isize)) as (*const u8));
                (*self).addr[bucket] = 0xccccccccu32;
                (*self).head[bucket] = 0xcccci32 as (u16);
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        memset(
            (*self).addr.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            0xcci32,
            ::std::mem::size_of::<[u32; 32768]>()
        );
        memset(
            (*self).head.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<[u16; 32768]>()
        );
    }
    memset(
        (*self).tiny_hash.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u8; 65536]>()
    );
    memset(
        (*self).free_slot_idx.as_mut_ptr(
        ) as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u16; 512]>()
    );
}

unsafe extern fn SelfH54(mut handle : *mut u8) -> *mut H54 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct4) as (*mut H54)
}

unsafe extern fn HashBytesH54(mut data : *const u8) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               data as (*const ::std::os::raw::c_void)
           ) << 64i32 - 8i32 * 7i32).wrapping_mul(
              kHashMul64
          );
    (h >> 64i32 - 20i32) as (u32)
}

unsafe extern fn PrepareH54(
    mut handle : *mut u8,
    mut one_shot : i32,
    mut input_size : usize,
    mut data : *const u8
) {
    let mut self : *mut H54 = SelfH54(handle);
    let mut partial_prepare_threshold
        : usize
        = (4i32 << 20i32 >> 7i32) as (usize);
    if one_shot != 0 && (input_size <= partial_prepare_threshold) {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < input_size {
            {
                let key
                    : u32
                    = HashBytesH54(&*data.offset(i as (isize)) as (*const u8));
                memset(
                    &mut (*self).buckets_[
                             key as (usize)
                         ] as (*mut u32) as (*mut ::std::os::raw::c_void),
                    0i32,
                    (4i32 as (usize)).wrapping_mul(::std::mem::size_of::<u32>())
                );
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        memset(
            &mut (*self).buckets_[
                     0i32 as (usize)
                 ] as (*mut u32) as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<[u32; 1048580]>()
        );
    }
}

unsafe extern fn PrepareH10(
    mut handle : *mut u8,
    mut one_shot : i32,
    mut input_size : usize,
    mut data : *const u8
) {
    let mut self : *mut H10 = SelfH10(handle);
    let mut invalid_pos : u32 = (*self).invalid_pos_;
    let mut i : u32;
    data;
    one_shot;
    input_size;
    i = 0i32 as (u32);
    while i < (1i32 << 17i32) as (u32) {
        {
            (*self).buckets_[i as (usize)] = invalid_pos;
        }
        i = i.wrapping_add(1 as (u32));
    }
}

unsafe extern fn HasherSetup(
    mut m : *mut MemoryManager,
    mut handle : *mut *mut u8,
    mut params : *mut BrotliEncoderParams,
    mut data : *const u8,
    mut position : usize,
    mut input_size : usize,
    mut is_last : i32
) {
    let mut self
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut common
        : *mut Struct4
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut Struct4);
    let mut one_shot
        : i32
        = (position == 0i32 as (usize) && (is_last != 0)) as (i32);
    if *handle == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        let mut alloc_size : usize;
        ChooseHasher(
            params as (*const BrotliEncoderParams),
            &mut (*params).hasher as (*mut BrotliHasherParams)
        );
        alloc_size = HasherSize(
                         params as (*const BrotliEncoderParams),
                         one_shot,
                         input_size
                     );
        self = if alloc_size != 0 {
                   BrotliAllocate(
                       m,
                       alloc_size.wrapping_mul(::std::mem::size_of::<u8>())
                   ) as (*mut u8)
               } else {
                   0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
               };
        if !(0i32 == 0) {
            return;
        }
        *handle = self;
        common = GetHasherCommon(self);
        (*common).params = (*params).hasher;
        let mut hasher_type : i32 = (*common).params.type_;
        if hasher_type == 2i32 {
            InitializeH2(*handle,params as (*const BrotliEncoderParams));
        }
        if hasher_type == 3i32 {
            InitializeH3(*handle,params as (*const BrotliEncoderParams));
        }
        if hasher_type == 4i32 {
            InitializeH4(*handle,params as (*const BrotliEncoderParams));
        }
        if hasher_type == 5i32 {
            InitializeH5(*handle,params as (*const BrotliEncoderParams));
        }
        if hasher_type == 6i32 {
            InitializeH6(*handle,params as (*const BrotliEncoderParams));
        }
        if hasher_type == 40i32 {
            InitializeH40(*handle,params as (*const BrotliEncoderParams));
        }
        if hasher_type == 41i32 {
            InitializeH41(*handle,params as (*const BrotliEncoderParams));
        }
        if hasher_type == 42i32 {
            InitializeH42(*handle,params as (*const BrotliEncoderParams));
        }
        if hasher_type == 54i32 {
            InitializeH54(*handle,params as (*const BrotliEncoderParams));
        }
        if hasher_type == 10i32 {
            InitializeH10(*handle,params as (*const BrotliEncoderParams));
        }
        HasherReset(*handle);
    }
    self = *handle;
    common = GetHasherCommon(self);
    if (*common).is_prepared_ == 0 {
        let mut hasher_type : i32 = (*common).params.type_;
        if hasher_type == 2i32 {
            PrepareH2(self,one_shot,input_size,data);
        }
        if hasher_type == 3i32 {
            PrepareH3(self,one_shot,input_size,data);
        }
        if hasher_type == 4i32 {
            PrepareH4(self,one_shot,input_size,data);
        }
        if hasher_type == 5i32 {
            PrepareH5(self,one_shot,input_size,data);
        }
        if hasher_type == 6i32 {
            PrepareH6(self,one_shot,input_size,data);
        }
        if hasher_type == 40i32 {
            PrepareH40(self,one_shot,input_size,data);
        }
        if hasher_type == 41i32 {
            PrepareH41(self,one_shot,input_size,data);
        }
        if hasher_type == 42i32 {
            PrepareH42(self,one_shot,input_size,data);
        }
        if hasher_type == 54i32 {
            PrepareH54(self,one_shot,input_size,data);
        }
        if hasher_type == 10i32 {
            PrepareH10(self,one_shot,input_size,data);
        }
        if position == 0i32 as (usize) {
            (*common).dict_num_lookups = 0i32 as (usize);
            (*common).dict_num_matches = 0i32 as (usize);
        }
        (*common).is_prepared_ = 1i32;
    }
}

unsafe extern fn StoreLookaheadH2() -> usize { 8i32 as (usize) }

unsafe extern fn StoreH2(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let key
        : u32
        = HashBytesH2(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let off
        : u32
        = (ix >> 3i32).wrapping_rem(1i32 as (usize)) as (u32);
    (*SelfH2(handle)).buckets_[
        key.wrapping_add(off) as (usize)
    ] = ix as (u32);
}

unsafe extern fn StoreLookaheadH3() -> usize { 8i32 as (usize) }

unsafe extern fn StoreH3(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let key
        : u32
        = HashBytesH3(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let off
        : u32
        = (ix >> 3i32).wrapping_rem(2i32 as (usize)) as (u32);
    (*SelfH3(handle)).buckets_[
        key.wrapping_add(off) as (usize)
    ] = ix as (u32);
}

unsafe extern fn StoreLookaheadH4() -> usize { 8i32 as (usize) }

unsafe extern fn StoreH4(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let key
        : u32
        = HashBytesH4(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let off
        : u32
        = (ix >> 3i32).wrapping_rem(4i32 as (usize)) as (u32);
    (*SelfH4(handle)).buckets_[
        key.wrapping_add(off) as (usize)
    ] = ix as (u32);
}

unsafe extern fn StoreLookaheadH5() -> usize { 4i32 as (usize) }

unsafe extern fn BucketsH5(mut self : *mut H5) -> *mut u32 {
    &mut *NumH5(self).offset(
              (*self).bucket_size_ as (isize)
          ) as (*mut u16) as (*mut u32)
}

unsafe extern fn StoreH5(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H5 = SelfH5(handle);
    let mut num : *mut u16 = NumH5(self);
    let key
        : u32
        = HashBytesH5(
              &*data.offset((ix & mask) as (isize)) as (*const u8),
              (*self).hash_shift_
          );
    let minor_ix
        : usize
        = (*num.offset(
                key as (isize)
            ) as (u32) & (*self).block_mask_) as (usize);
    let offset
        : usize
        = minor_ix.wrapping_add(
              (key << (*GetHasherCommon(handle)).params.block_bits) as (usize)
          );
    *BucketsH5(self).offset(offset as (isize)) = ix as (u32);
    {
        let _rhs = 1;
        let _lhs = &mut *num.offset(key as (isize));
        *_lhs = (*_lhs as (i32) + _rhs) as (u16);
    }
}

unsafe extern fn StoreLookaheadH6() -> usize { 8i32 as (usize) }

unsafe extern fn BucketsH6(mut self : *mut H6) -> *mut u32 {
    &mut *NumH6(self).offset(
              (*self).bucket_size_ as (isize)
          ) as (*mut u16) as (*mut u32)
}

unsafe extern fn StoreH6(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H6 = SelfH6(handle);
    let mut num : *mut u16 = NumH6(self);
    let key
        : u32
        = HashBytesH6(
              &*data.offset((ix & mask) as (isize)) as (*const u8),
              (*self).hash_mask_,
              (*self).hash_shift_
          );
    let minor_ix
        : usize
        = (*num.offset(
                key as (isize)
            ) as (u32) & (*self).block_mask_) as (usize);
    let offset
        : usize
        = minor_ix.wrapping_add(
              (key << (*GetHasherCommon(handle)).params.block_bits) as (usize)
          );
    *BucketsH6(self).offset(offset as (isize)) = ix as (u32);
    {
        let _rhs = 1;
        let _lhs = &mut *num.offset(key as (isize));
        *_lhs = (*_lhs as (i32) + _rhs) as (u16);
    }
}

unsafe extern fn StoreLookaheadH40() -> usize { 4i32 as (usize) }

unsafe extern fn StoreH40(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H40 = SelfH40(handle);
    let key
        : usize
        = HashBytesH40(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let bank : usize = key & (1i32 - 1i32) as (usize);
    let idx
        : usize
        = (({
                let _rhs = 1;
                let _lhs = &mut (*self).free_slot_idx[bank];
                let _old = *_lhs;
                *_lhs = (*_lhs as (i32) + _rhs) as (u16);
                _old
            }) as (i32) & 65536i32 - 1i32) as (usize);
    let mut delta
        : usize
        = ix.wrapping_sub((*self).addr[key] as (usize));
    (*self).tiny_hash[ix as (u16) as (usize)] = key as (u8);
    if delta > 0xffffi32 as (usize) {
        delta = if 0i32 != 0 { 0i32 } else { 0xffffi32 } as (usize);
    }
    (*self).banks[bank].slots[idx].delta = delta as (u16);
    (*self).banks[bank].slots[idx].next = (*self).head[key];
    (*self).addr[key] = ix as (u32);
    (*self).head[key] = idx as (u16);
}

unsafe extern fn StoreLookaheadH41() -> usize { 4i32 as (usize) }

unsafe extern fn StoreH41(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H41 = SelfH41(handle);
    let key
        : usize
        = HashBytesH41(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let bank : usize = key & (1i32 - 1i32) as (usize);
    let idx
        : usize
        = (({
                let _rhs = 1;
                let _lhs = &mut (*self).free_slot_idx[bank];
                let _old = *_lhs;
                *_lhs = (*_lhs as (i32) + _rhs) as (u16);
                _old
            }) as (i32) & 65536i32 - 1i32) as (usize);
    let mut delta
        : usize
        = ix.wrapping_sub((*self).addr[key] as (usize));
    (*self).tiny_hash[ix as (u16) as (usize)] = key as (u8);
    if delta > 0xffffi32 as (usize) {
        delta = if 0i32 != 0 { 0i32 } else { 0xffffi32 } as (usize);
    }
    (*self).banks[bank].slots[idx].delta = delta as (u16);
    (*self).banks[bank].slots[idx].next = (*self).head[key];
    (*self).addr[key] = ix as (u32);
    (*self).head[key] = idx as (u16);
}

unsafe extern fn StoreLookaheadH42() -> usize { 4i32 as (usize) }

unsafe extern fn StoreH42(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H42 = SelfH42(handle);
    let key
        : usize
        = HashBytesH42(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let bank : usize = key & (512i32 - 1i32) as (usize);
    let idx
        : usize
        = (({
                let _rhs = 1;
                let _lhs = &mut (*self).free_slot_idx[bank];
                let _old = *_lhs;
                *_lhs = (*_lhs as (i32) + _rhs) as (u16);
                _old
            }) as (i32) & 512i32 - 1i32) as (usize);
    let mut delta
        : usize
        = ix.wrapping_sub((*self).addr[key] as (usize));
    (*self).tiny_hash[ix as (u16) as (usize)] = key as (u8);
    if delta > 0xffffi32 as (usize) {
        delta = if 0i32 != 0 { 0i32 } else { 0xffffi32 } as (usize);
    }
    (*self).banks[bank].slots[idx].delta = delta as (u16);
    (*self).banks[bank].slots[idx].next = (*self).head[key];
    (*self).addr[key] = ix as (u32);
    (*self).head[key] = idx as (u16);
}

unsafe extern fn StoreLookaheadH54() -> usize { 8i32 as (usize) }

unsafe extern fn StoreH54(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let key
        : u32
        = HashBytesH54(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let off
        : u32
        = (ix >> 3i32).wrapping_rem(4i32 as (usize)) as (u32);
    (*SelfH54(handle)).buckets_[
        key.wrapping_add(off) as (usize)
    ] = ix as (u32);
}

unsafe extern fn StoreLookaheadH10() -> usize { 128i32 as (usize) }

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BackwardMatch {
    pub distance : u32,
    pub length_and_code : u32,
}

unsafe extern fn HashBytesH10(mut data : *const u8) -> u32 {
    let mut h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data as (*const ::std::os::raw::c_void)
          ).wrapping_mul(
              kHashMul32
          );
    h >> 32i32 - 17i32
}

unsafe extern fn ForestH10(mut self : *mut H10) -> *mut u32 {
    &mut *self.offset(1i32 as (isize)) as (*mut H10) as (*mut u32)
}

unsafe extern fn LeftChildIndexH10(
    mut self : *mut H10, pos : usize
) -> usize {
    (2i32 as (usize)).wrapping_mul(pos & (*self).window_mask_)
}

unsafe extern fn RightChildIndexH10(
    mut self : *mut H10, pos : usize
) -> usize {
    (2i32 as (usize)).wrapping_mul(
        pos & (*self).window_mask_
    ).wrapping_add(
        1i32 as (usize)
    )
}

unsafe extern fn unopt_ctzll(mut val : usize) -> u8 {
    let mut cnt : u8 = 0i32 as (u8);
    while val & 1i32 as (usize) == 0i32 as (usize) {
        val = val >> 1i32;
        cnt = (cnt as (i32) + 1) as (u8);
    }
    cnt
}

unsafe extern fn FindMatchLengthWithLimit(
    mut s1 : *const u8, mut s2 : *const u8, mut limit : usize
) -> usize {
    let mut matched : usize = 0i32 as (usize);
    let mut limit2
        : usize
        = (limit >> 3i32).wrapping_add(1i32 as (usize));
    while {
              limit2 = limit2.wrapping_sub(1 as (usize));
              limit2
          } != 0 {
        if BROTLI_UNALIGNED_LOAD64(
               s2 as (*const ::std::os::raw::c_void)
           ) == BROTLI_UNALIGNED_LOAD64(
                    s1.offset(matched as (isize)) as (*const ::std::os::raw::c_void)
                ) {
            s2 = s2.offset(8i32 as (isize));
            matched = matched.wrapping_add(8i32 as (usize));
        } else {
            let mut x
                : usize
                = BROTLI_UNALIGNED_LOAD64(
                      s2 as (*const ::std::os::raw::c_void)
                  ) ^ BROTLI_UNALIGNED_LOAD64(
                          s1.offset(matched as (isize)) as (*const ::std::os::raw::c_void)
                      );
            let mut matching_bits : usize = unopt_ctzll(x) as (usize);
            matched = matched.wrapping_add(matching_bits >> 3i32);
            return matched;
        }
    }
    limit = (limit & 7i32 as (usize)).wrapping_add(1i32 as (usize));
    while {
              limit = limit.wrapping_sub(1 as (usize));
              limit
          } != 0 {
        if *s1.offset(matched as (isize)) as (i32) == *s2 as (i32) {
            s2 = s2.offset(1 as (isize));
            matched = matched.wrapping_add(1 as (usize));
        } else {
            return matched;
        }
    }
    matched
}

unsafe extern fn InitBackwardMatch(
    mut self : *mut BackwardMatch, mut dist : usize, mut len : usize
) {
    (*self).distance = dist as (u32);
    (*self).length_and_code = (len << 5i32) as (u32);
}

unsafe extern fn StoreAndFindMatchesH10(
    mut self : *mut H10,
    data : *const u8,
    cur_ix : usize,
    ring_buffer_mask : usize,
    max_length : usize,
    max_backward : usize,
    best_len : *mut usize,
    mut matches : *mut BackwardMatch
) -> *mut BackwardMatch {
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let max_comp_len
        : usize
        = brotli_min_size_t(max_length,128i32 as (usize));
    let should_reroot_tree
        : i32
        = if !!(max_length >= 128i32 as (usize)) { 1i32 } else { 0i32 };
    let key
        : u32
        = HashBytesH10(
              &*data.offset(cur_ix_masked as (isize)) as (*const u8)
          );
    let mut forest : *mut u32 = ForestH10(self);
    let mut prev_ix
        : usize
        = (*self).buckets_[key as (usize)] as (usize);
    let mut node_left : usize = LeftChildIndexH10(self,cur_ix);
    let mut node_right : usize = RightChildIndexH10(self,cur_ix);
    let mut best_len_left : usize = 0i32 as (usize);
    let mut best_len_right : usize = 0i32 as (usize);
    let mut depth_remaining : usize;
    if should_reroot_tree != 0 {
        (*self).buckets_[key as (usize)] = cur_ix as (u32);
    }
    depth_remaining = 64i32 as (usize);
    'break45: loop {
        {
            let backward : usize = cur_ix.wrapping_sub(prev_ix);
            let prev_ix_masked : usize = prev_ix & ring_buffer_mask;
            if backward == 0i32 as (usize) || backward > max_backward || depth_remaining == 0i32 as (usize) {
                if should_reroot_tree != 0 {
                    *forest.offset(node_left as (isize)) = (*self).invalid_pos_;
                    *forest.offset(node_right as (isize)) = (*self).invalid_pos_;
                }
                {
                    if 1337i32 != 0 {
                        break 'break45;
                    }
                }
            }
            {
                let cur_len
                    : usize
                    = brotli_min_size_t(best_len_left,best_len_right);
                let mut len : usize;
                0i32;
                len = cur_len.wrapping_add(
                          FindMatchLengthWithLimit(
                              &*data.offset(
                                    cur_ix_masked.wrapping_add(cur_len) as (isize)
                                ) as (*const u8),
                              &*data.offset(
                                    prev_ix_masked.wrapping_add(cur_len) as (isize)
                                ) as (*const u8),
                              max_length.wrapping_sub(cur_len)
                          )
                      );
                0i32;
                if !matches.is_null() && (len > *best_len) {
                    *best_len = len;
                    InitBackwardMatch(
                        {
                            let _old = matches;
                            matches = matches.offset(1 as (isize));
                            _old
                        },
                        backward,
                        len
                    );
                }
                if len >= max_comp_len {
                    if should_reroot_tree != 0 {
                        *forest.offset(node_left as (isize)) = *forest.offset(
                                                                    LeftChildIndexH10(
                                                                        self,
                                                                        prev_ix
                                                                    ) as (isize)
                                                                );
                        *forest.offset(node_right as (isize)) = *forest.offset(
                                                                     RightChildIndexH10(
                                                                         self,
                                                                         prev_ix
                                                                     ) as (isize)
                                                                 );
                    }
                    {
                        if 1337i32 != 0 {
                            break 'break45;
                        }
                    }
                }
                if *data.offset(
                        cur_ix_masked.wrapping_add(len) as (isize)
                    ) as (i32) > *data.offset(
                                      prev_ix_masked.wrapping_add(len) as (isize)
                                  ) as (i32) {
                    best_len_left = len;
                    if should_reroot_tree != 0 {
                        *forest.offset(node_left as (isize)) = prev_ix as (u32);
                    }
                    node_left = RightChildIndexH10(self,prev_ix);
                    prev_ix = *forest.offset(node_left as (isize)) as (usize);
                } else {
                    best_len_right = len;
                    if should_reroot_tree != 0 {
                        *forest.offset(node_right as (isize)) = prev_ix as (u32);
                    }
                    node_right = LeftChildIndexH10(self,prev_ix);
                    prev_ix = *forest.offset(node_right as (isize)) as (usize);
                }
            }
        }
        depth_remaining = depth_remaining.wrapping_sub(1 as (usize));
    }
    matches
}

unsafe extern fn StoreH10(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H10 = SelfH10(handle);
    let max_backward
        : usize
        = (*self).window_mask_.wrapping_sub(16i32 as (usize)).wrapping_add(
              1i32 as (usize)
          );
    StoreAndFindMatchesH10(
        self,
        data,
        ix,
        mask,
        128i32 as (usize),
        max_backward,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut usize),
        0i32 as (*mut ::std::os::raw::c_void) as (*mut BackwardMatch)
    );
}

unsafe extern fn HasherPrependCustomDictionary(
    mut m : *mut MemoryManager,
    mut handle : *mut *mut u8,
    mut params : *mut BrotliEncoderParams,
    size : usize,
    mut dict : *const u8
) {
    let mut overlap : usize;
    let mut i : usize;
    let mut self : *mut u8;
    HasherSetup(m,handle,params,dict,0i32 as (usize),size,0i32);
    if !(0i32 == 0) {
        return;
    }
    self = *handle;
    let mut hasher_type : i32 = (*GetHasherCommon(self)).params.type_;
    if hasher_type == 2i32 {
        overlap = StoreLookaheadH2().wrapping_sub(1i32 as (usize));
        i = 0i32 as (usize);
        while i.wrapping_add(overlap) < size {
            {
                StoreH2(self,dict,!(0i32 as (usize)),i);
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    if hasher_type == 3i32 {
        overlap = StoreLookaheadH3().wrapping_sub(1i32 as (usize));
        i = 0i32 as (usize);
        while i.wrapping_add(overlap) < size {
            {
                StoreH3(self,dict,!(0i32 as (usize)),i);
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    if hasher_type == 4i32 {
        overlap = StoreLookaheadH4().wrapping_sub(1i32 as (usize));
        i = 0i32 as (usize);
        while i.wrapping_add(overlap) < size {
            {
                StoreH4(self,dict,!(0i32 as (usize)),i);
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    if hasher_type == 5i32 {
        overlap = StoreLookaheadH5().wrapping_sub(1i32 as (usize));
        i = 0i32 as (usize);
        while i.wrapping_add(overlap) < size {
            {
                StoreH5(self,dict,!(0i32 as (usize)),i);
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    if hasher_type == 6i32 {
        overlap = StoreLookaheadH6().wrapping_sub(1i32 as (usize));
        i = 0i32 as (usize);
        while i.wrapping_add(overlap) < size {
            {
                StoreH6(self,dict,!(0i32 as (usize)),i);
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    if hasher_type == 40i32 {
        overlap = StoreLookaheadH40().wrapping_sub(1i32 as (usize));
        i = 0i32 as (usize);
        while i.wrapping_add(overlap) < size {
            {
                StoreH40(self,dict,!(0i32 as (usize)),i);
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    if hasher_type == 41i32 {
        overlap = StoreLookaheadH41().wrapping_sub(1i32 as (usize));
        i = 0i32 as (usize);
        while i.wrapping_add(overlap) < size {
            {
                StoreH41(self,dict,!(0i32 as (usize)),i);
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    if hasher_type == 42i32 {
        overlap = StoreLookaheadH42().wrapping_sub(1i32 as (usize));
        i = 0i32 as (usize);
        while i.wrapping_add(overlap) < size {
            {
                StoreH42(self,dict,!(0i32 as (usize)),i);
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    if hasher_type == 54i32 {
        overlap = StoreLookaheadH54().wrapping_sub(1i32 as (usize));
        i = 0i32 as (usize);
        while i.wrapping_add(overlap) < size {
            {
                StoreH54(self,dict,!(0i32 as (usize)),i);
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    if hasher_type == 10i32 {
        overlap = StoreLookaheadH10().wrapping_sub(1i32 as (usize));
        i = 0i32 as (usize);
        while i.wrapping_add(overlap) < size {
            {
                StoreH10(self,dict,!(0i32 as (usize)),i);
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderSetCustomDictionary(
    mut s : *mut BrotliEncoderStateStruct,
    mut size : usize,
    mut dict : *const u8
) {
    let mut max_dict_size
        : usize
        = (1i32 as (usize) << (*s).params.lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let mut dict_size : usize = size;
    let mut m
        : *mut MemoryManager
        = &mut (*s).memory_manager_ as (*mut MemoryManager);
    if EnsureInitialized(s) == 0 {
        return;
    }
    if dict_size == 0i32 as (usize) || (*s).params.quality == 0i32 || (*s).params.quality == 1i32 {
        return;
    }
    if size > max_dict_size {
        dict = dict.offset(size.wrapping_sub(max_dict_size) as (isize));
        dict_size = max_dict_size;
    }
    CopyInputToRingBuffer(s,dict_size,dict);
    (*s).last_flush_pos_ = dict_size;
    (*s).last_processed_pos_ = dict_size;
    if dict_size > 0i32 as (usize) {
        (*s).prev_byte_ = *dict.offset(
                               dict_size.wrapping_sub(1i32 as (usize)) as (isize)
                           );
    }
    if dict_size > 1i32 as (usize) {
        (*s).prev_byte2_ = *dict.offset(
                                dict_size.wrapping_sub(2i32 as (usize)) as (isize)
                            );
    }
    HasherPrependCustomDictionary(
        m,
        &mut (*s).hasher_ as (*mut *mut u8),
        &mut (*s).params as (*mut BrotliEncoderParams),
        dict_size,
        dict
    );
    if !(0i32 == 0) { }
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderMaxCompressedSize(
    mut input_size : usize
) -> usize {
    let mut num_large_blocks : usize = input_size >> 24i32;
    let mut tail
        : usize
        = input_size.wrapping_sub(num_large_blocks << 24i32);
    let mut tail_overhead
        : usize
        = (if tail > (1i32 << 20i32) as (usize) {
               4i32
           } else {
               3i32
           }) as (usize);
    let mut overhead
        : usize
        = (2i32 as (usize)).wrapping_add(
              (4i32 as (usize)).wrapping_mul(num_large_blocks)
          ).wrapping_add(
              tail_overhead
          ).wrapping_add(
              1i32 as (usize)
          );
    let mut result : usize = input_size.wrapping_add(overhead);
    if input_size == 0i32 as (usize) {
        return 1i32 as (usize);
    }
    if result < input_size { 0i32 as (usize) } else { result }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length : [u8; 32],
    pub offsets_by_length : [u32; 32],
    pub data : [u8; 122784],
}

unsafe extern fn HashTypeLengthH2() -> usize { 8i32 as (usize) }

unsafe extern fn StitchToPreviousBlockH2(
    mut handle : *mut u8,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize
) { if num_bytes >= HashTypeLengthH2().wrapping_sub(
                        1i32 as (usize)
                    ) && (position >= 3i32 as (usize)) {
        StoreH2(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3i32 as (usize))
        );
        StoreH2(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2i32 as (usize))
        );
        StoreH2(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1i32 as (usize))
        );
    }
}

unsafe extern fn HashTypeLengthH3() -> usize { 8i32 as (usize) }

unsafe extern fn StitchToPreviousBlockH3(
    mut handle : *mut u8,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize
) { if num_bytes >= HashTypeLengthH3().wrapping_sub(
                        1i32 as (usize)
                    ) && (position >= 3i32 as (usize)) {
        StoreH3(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3i32 as (usize))
        );
        StoreH3(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2i32 as (usize))
        );
        StoreH3(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1i32 as (usize))
        );
    }
}

unsafe extern fn HashTypeLengthH4() -> usize { 8i32 as (usize) }

unsafe extern fn StitchToPreviousBlockH4(
    mut handle : *mut u8,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize
) { if num_bytes >= HashTypeLengthH4().wrapping_sub(
                        1i32 as (usize)
                    ) && (position >= 3i32 as (usize)) {
        StoreH4(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3i32 as (usize))
        );
        StoreH4(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2i32 as (usize))
        );
        StoreH4(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1i32 as (usize))
        );
    }
}

unsafe extern fn HashTypeLengthH5() -> usize { 4i32 as (usize) }

unsafe extern fn StitchToPreviousBlockH5(
    mut handle : *mut u8,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize
) { if num_bytes >= HashTypeLengthH5().wrapping_sub(
                        1i32 as (usize)
                    ) && (position >= 3i32 as (usize)) {
        StoreH5(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3i32 as (usize))
        );
        StoreH5(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2i32 as (usize))
        );
        StoreH5(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1i32 as (usize))
        );
    }
}

unsafe extern fn HashTypeLengthH6() -> usize { 8i32 as (usize) }

unsafe extern fn StitchToPreviousBlockH6(
    mut handle : *mut u8,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize
) { if num_bytes >= HashTypeLengthH6().wrapping_sub(
                        1i32 as (usize)
                    ) && (position >= 3i32 as (usize)) {
        StoreH6(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3i32 as (usize))
        );
        StoreH6(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2i32 as (usize))
        );
        StoreH6(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1i32 as (usize))
        );
    }
}

unsafe extern fn HashTypeLengthH40() -> usize { 4i32 as (usize) }

unsafe extern fn StitchToPreviousBlockH40(
    mut handle : *mut u8,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ring_buffer_mask : usize
) { if num_bytes >= HashTypeLengthH40().wrapping_sub(
                        1i32 as (usize)
                    ) && (position >= 3i32 as (usize)) {
        StoreH40(
            handle,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(3i32 as (usize))
        );
        StoreH40(
            handle,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(2i32 as (usize))
        );
        StoreH40(
            handle,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(1i32 as (usize))
        );
    }
}

unsafe extern fn HashTypeLengthH41() -> usize { 4i32 as (usize) }

unsafe extern fn StitchToPreviousBlockH41(
    mut handle : *mut u8,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ring_buffer_mask : usize
) { if num_bytes >= HashTypeLengthH41().wrapping_sub(
                        1i32 as (usize)
                    ) && (position >= 3i32 as (usize)) {
        StoreH41(
            handle,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(3i32 as (usize))
        );
        StoreH41(
            handle,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(2i32 as (usize))
        );
        StoreH41(
            handle,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(1i32 as (usize))
        );
    }
}

unsafe extern fn HashTypeLengthH42() -> usize { 4i32 as (usize) }

unsafe extern fn StitchToPreviousBlockH42(
    mut handle : *mut u8,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ring_buffer_mask : usize
) { if num_bytes >= HashTypeLengthH42().wrapping_sub(
                        1i32 as (usize)
                    ) && (position >= 3i32 as (usize)) {
        StoreH42(
            handle,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(3i32 as (usize))
        );
        StoreH42(
            handle,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(2i32 as (usize))
        );
        StoreH42(
            handle,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(1i32 as (usize))
        );
    }
}

unsafe extern fn HashTypeLengthH54() -> usize { 8i32 as (usize) }

unsafe extern fn StitchToPreviousBlockH54(
    mut handle : *mut u8,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize
) { if num_bytes >= HashTypeLengthH54().wrapping_sub(
                        1i32 as (usize)
                    ) && (position >= 3i32 as (usize)) {
        StoreH54(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3i32 as (usize))
        );
        StoreH54(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2i32 as (usize))
        );
        StoreH54(
            handle,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1i32 as (usize))
        );
    }
}

unsafe extern fn HashTypeLengthH10() -> usize { 4i32 as (usize) }

unsafe extern fn brotli_max_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a > b { a } else { b }
}

unsafe extern fn StitchToPreviousBlockH10(
    mut handle : *mut u8,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize
) {
    let mut self : *mut H10 = SelfH10(handle);
    if num_bytes >= HashTypeLengthH10().wrapping_sub(
                        1i32 as (usize)
                    ) && (position >= 128i32 as (usize)) {
        let i_start
            : usize
            = position.wrapping_sub(128i32 as (usize)).wrapping_add(
                  1i32 as (usize)
              );
        let i_end
            : usize
            = brotli_min_size_t(position,i_start.wrapping_add(num_bytes));
        let mut i : usize;
        i = i_start;
        while i < i_end {
            {
                let max_backward
                    : usize
                    = (*self).window_mask_.wrapping_sub(
                          brotli_max_size_t(
                              (16i32 - 1i32) as (usize),
                              position.wrapping_sub(i)
                          )
                      );
                StoreAndFindMatchesH10(
                    self,
                    ringbuffer,
                    i,
                    ringbuffer_mask,
                    128i32 as (usize),
                    max_backward,
                    0i32 as (*mut ::std::os::raw::c_void) as (*mut usize),
                    0i32 as (*mut ::std::os::raw::c_void) as (*mut BackwardMatch)
                );
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
}

unsafe extern fn InitOrStitchToPreviousBlock(
    mut m : *mut MemoryManager,
    mut handle : *mut *mut u8,
    mut data : *const u8,
    mut mask : usize,
    mut params : *mut BrotliEncoderParams,
    mut position : usize,
    mut input_size : usize,
    mut is_last : i32
) {
    let mut self : *mut u8;
    HasherSetup(m,handle,params,data,position,input_size,is_last);
    if !(0i32 == 0) {
        return;
    }
    self = *handle;
    let mut hasher_type : i32 = (*GetHasherCommon(self)).params.type_;
    if hasher_type == 2i32 {
        StitchToPreviousBlockH2(self,input_size,position,data,mask);
    }
    if hasher_type == 3i32 {
        StitchToPreviousBlockH3(self,input_size,position,data,mask);
    }
    if hasher_type == 4i32 {
        StitchToPreviousBlockH4(self,input_size,position,data,mask);
    }
    if hasher_type == 5i32 {
        StitchToPreviousBlockH5(self,input_size,position,data,mask);
    }
    if hasher_type == 6i32 {
        StitchToPreviousBlockH6(self,input_size,position,data,mask);
    }
    if hasher_type == 40i32 {
        StitchToPreviousBlockH40(self,input_size,position,data,mask);
    }
    if hasher_type == 41i32 {
        StitchToPreviousBlockH41(self,input_size,position,data,mask);
    }
    if hasher_type == 42i32 {
        StitchToPreviousBlockH42(self,input_size,position,data,mask);
    }
    if hasher_type == 54i32 {
        StitchToPreviousBlockH54(self,input_size,position,data,mask);
    }
    if hasher_type == 10i32 {
        StitchToPreviousBlockH10(self,input_size,position,data,mask);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Struct49 {
    pub cost : f32,
    pub next : u32,
    pub shortcut : u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ZopfliNode {
    pub length : u32,
    pub distance : u32,
    pub insert_length : u32,
    pub u : Struct49,
}

unsafe extern fn Log2FloorNonZero(mut n : usize) -> u32 {
    let mut result : u32 = 0i32 as (u32);
    while {
              n = n >> 1i32;
              n
          } != 0 {
        result = result.wrapping_add(1 as (u32));
    }
    result
}

unsafe extern fn GetInsertLengthCode(
    mut insertlen : usize
) -> u16 {
    if insertlen < 6i32 as (usize) {
        insertlen as (u16)
    } else if insertlen < 130i32 as (usize) {
        let mut nbits
            : u32
            = Log2FloorNonZero(
                  insertlen.wrapping_sub(2i32 as (usize))
              ).wrapping_sub(
                  1u32
              );
        ((nbits << 1i32) as (usize)).wrapping_add(
            insertlen.wrapping_sub(2i32 as (usize)) >> nbits
        ).wrapping_add(
            2i32 as (usize)
        ) as (u16)
    } else if insertlen < 2114i32 as (usize) {
        Log2FloorNonZero(
            insertlen.wrapping_sub(66i32 as (usize))
        ).wrapping_add(
            10i32 as (u32)
        ) as (u16)
    } else if insertlen < 6210i32 as (usize) {
        21u32 as (u16)
    } else if insertlen < 22594i32 as (usize) {
        22u32 as (u16)
    } else {
        23u32 as (u16)
    }
}

unsafe extern fn GetCopyLengthCode(mut copylen : usize) -> u16 {
    if copylen < 10i32 as (usize) {
        copylen.wrapping_sub(2i32 as (usize)) as (u16)
    } else if copylen < 134i32 as (usize) {
        let mut nbits
            : u32
            = Log2FloorNonZero(
                  copylen.wrapping_sub(6i32 as (usize))
              ).wrapping_sub(
                  1u32
              );
        ((nbits << 1i32) as (usize)).wrapping_add(
            copylen.wrapping_sub(6i32 as (usize)) >> nbits
        ).wrapping_add(
            4i32 as (usize)
        ) as (u16)
    } else if copylen < 2118i32 as (usize) {
        Log2FloorNonZero(
            copylen.wrapping_sub(70i32 as (usize))
        ).wrapping_add(
            12i32 as (u32)
        ) as (u16)
    } else {
        23u32 as (u16)
    }
}

unsafe extern fn CombineLengthCodes(
    mut inscode : u16, mut copycode : u16, mut use_last_distance : i32
) -> u16 {
    let mut bits64
        : u16
        = (copycode as (u32) & 0x7u32 | (inscode as (u32) & 0x7u32) << 3i32) as (u16);
    if use_last_distance != 0 && (inscode as (i32) < 8i32) && (copycode as (i32) < 16i32) {
        if copycode as (i32) < 8i32 {
            bits64
        } else {
            let mut s64 : u16 = 64i32 as (u16);
            (bits64 as (i32) | s64 as (i32)) as (u16)
        }
    } else {
        let mut offset
            : i32
            = 2i32 * ((copycode as (i32) >> 3i32) + 3i32 * (inscode as (i32) >> 3i32));
        offset = (offset << 5i32) + 0x40i32 + (0x520d40i32 >> offset & 0xc0i32);
        (offset as (u16) as (i32) | bits64 as (i32)) as (u16)
    }
}

unsafe extern fn GetLengthCode(
    mut insertlen : usize,
    mut copylen : usize,
    mut use_last_distance : i32,
    mut code : *mut u16
) {
    let mut inscode : u16 = GetInsertLengthCode(insertlen);
    let mut copycode : u16 = GetCopyLengthCode(copylen);
    *code = CombineLengthCodes(inscode,copycode,use_last_distance);
}

unsafe extern fn InitInsertCommand(
    mut self : *mut Command, mut insertlen : usize
) {
    (*self).insert_len_ = insertlen as (u32);
    (*self).copy_len_ = (4i32 << 24i32) as (u32);
    (*self).dist_extra_ = 0i32 as (u32);
    (*self).dist_prefix_ = 16i32 as (u16);
    GetLengthCode(
        insertlen,
        4i32 as (usize),
        0i32,
        &mut (*self).cmd_prefix_ as (*mut u16)
    );
}

unsafe extern fn BROTLI_UNALIGNED_STORE64(
    mut p : *mut ::std::os::raw::c_void, mut v : usize
) {
    memcpy(
        p,
        &mut v as (*mut usize) as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<usize>()
    );
}

unsafe extern fn BrotliWriteBits(
    mut n_bits : usize,
    mut bits : usize,
    mut pos : *mut usize,
    mut array : *mut u8
) {
    let mut p
        : *mut u8
        = &mut *array.offset((*pos >> 3i32) as (isize)) as (*mut u8);
    let mut v : usize = *p as (usize);
    0i32;
    0i32;
    v = v | bits << (*pos & 7i32 as (usize));
    BROTLI_UNALIGNED_STORE64(p as (*mut ::std::os::raw::c_void),v);
    *pos = (*pos).wrapping_add(n_bits);
}

unsafe extern fn FastLog2(mut v : usize) -> f64 {
    if v < ::std::mem::size_of::<[f32; 256]>().wrapping_div(
               ::std::mem::size_of::<f32>()
           ) {
        return kLog2Table[v] as (f64);
    }
    log2(v as (f64))
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
    while population < population_end {
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

unsafe extern fn ShouldCompress(
    mut data : *const u8,
    mask : usize,
    last_flush_pos : usize,
    bytes : usize,
    num_literals : usize,
    num_commands : usize
) -> i32 {
    if num_commands < (bytes >> 8i32).wrapping_add(2i32 as (usize)) {
        if num_literals as (f64) > 0.99f64 * bytes as (f64) {
            let mut literal_histo
                : [u32; 256]
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
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
                      0i32 as (u32),
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
            static kSampleRate : u32 = 13i32 as (u32);
            static kMinEntropy : f64 = 7.92f64;
            let bit_cost_threshold
                : f64
                = bytes as (f64) * kMinEntropy / kSampleRate as (f64);
            let mut t
                : usize
                = bytes.wrapping_add(kSampleRate as (usize)).wrapping_sub(
                      1i32 as (usize)
                  ).wrapping_div(
                      kSampleRate as (usize)
                  );
            let mut pos : u32 = last_flush_pos as (u32);
            let mut i : usize;
            i = 0i32 as (usize);
            while i < t {
                {
                    {
                        let _rhs = 1;
                        let _lhs
                            = &mut literal_histo[
                                       *data.offset((pos as (usize) & mask) as (isize)) as (usize)
                                   ];
                        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                    }
                    pos = pos.wrapping_add(kSampleRate);
                }
                i = i.wrapping_add(1 as (usize));
            }
            if BitsEntropy(
                   literal_histo.as_mut_ptr() as (*const u32),
                   256i32 as (usize)
               ) > bit_cost_threshold {
                return 0i32;
            }
        }
    }
    1i32
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

unsafe extern fn InitMetaBlockSplit(mut mb : *mut MetaBlockSplit) {
    BrotliInitBlockSplit(
        &mut (*mb).literal_split as (*mut BlockSplit)
    );
    BrotliInitBlockSplit(
        &mut (*mb).command_split as (*mut BlockSplit)
    );
    BrotliInitBlockSplit(
        &mut (*mb).distance_split as (*mut BlockSplit)
    );
    (*mb).literal_context_map = 0i32 as (*mut u32);
    (*mb).literal_context_map_size = 0i32 as (usize);
    (*mb).distance_context_map = 0i32 as (*mut u32);
    (*mb).distance_context_map_size = 0i32 as (usize);
    (*mb).literal_histograms = 0i32 as (*mut HistogramLiteral);
    (*mb).literal_histograms_size = 0i32 as (usize);
    (*mb).command_histograms = 0i32 as (*mut HistogramCommand);
    (*mb).command_histograms_size = 0i32 as (usize);
    (*mb).distance_histograms = 0i32 as (*mut HistogramDistance);
    (*mb).distance_histograms_size = 0i32 as (usize);
}

unsafe extern fn DestroyMetaBlockSplit(
    mut m : *mut MemoryManager, mut mb : *mut MetaBlockSplit
) {
    BrotliDestroyBlockSplit(
        m,
        &mut (*mb).literal_split as (*mut BlockSplit)
    );
    BrotliDestroyBlockSplit(
        m,
        &mut (*mb).command_split as (*mut BlockSplit)
    );
    BrotliDestroyBlockSplit(
        m,
        &mut (*mb).distance_split as (*mut BlockSplit)
    );
    {
        BrotliFree(
            m,
            (*mb).literal_context_map as (*mut ::std::os::raw::c_void)
        );
        (*mb).literal_context_map = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
    }
    {
        BrotliFree(
            m,
            (*mb).distance_context_map as (*mut ::std::os::raw::c_void)
        );
        (*mb).distance_context_map = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
    }
    {
        BrotliFree(
            m,
            (*mb).literal_histograms as (*mut ::std::os::raw::c_void)
        );
        (*mb).literal_histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral);
    }
    {
        BrotliFree(
            m,
            (*mb).command_histograms as (*mut ::std::os::raw::c_void)
        );
        (*mb).command_histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand);
    }
    {
        BrotliFree(
            m,
            (*mb).distance_histograms as (*mut ::std::os::raw::c_void)
        );
        (*mb).distance_histograms = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance);
    }
}

unsafe extern fn BrotliCompressBufferQuality10(
    mut lgwin : i32,
    mut input_size : usize,
    mut input_buffer : *const u8,
    mut encoded_size : *mut usize,
    mut encoded_buffer : *mut u8
) -> i32 {
    let mut memory_manager : MemoryManager;
    let mut m
        : *mut MemoryManager
        = &mut memory_manager as (*mut MemoryManager);
    let mask : usize = !(0i32 as (usize)) >> 1i32;
    let max_backward_limit
        : usize
        = (1i32 as (usize) << lgwin).wrapping_sub(16i32 as (usize));
    let mut dist_cache : [i32; 4] = [ 4i32, 11i32, 15i32, 16i32 ];
    let mut saved_dist_cache
        : [i32; 4]
        = [ 4i32, 11i32, 15i32, 16i32 ];
    let mut ok : i32 = 1i32;
    let max_out_size : usize = *encoded_size;
    let mut total_out_size : usize = 0i32 as (usize);
    let mut last_byte : u8;
    let mut last_byte_bits : u8;
    let mut hasher
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let hasher_eff_size
        : usize
        = brotli_min_size_t(
              input_size,
              max_backward_limit.wrapping_add(16i32 as (usize))
          );
    let mut params : BrotliEncoderParams;
    let mut dictionary
        : *const BrotliDictionary
        = BrotliGetDictionary();
    let lgmetablock : i32 = brotli_min_int(24i32,lgwin + 1i32);
    let mut max_block_size : usize;
    let max_metablock_size : usize = 1i32 as (usize) << lgmetablock;
    let max_literals_per_metablock
        : usize
        = max_metablock_size.wrapping_div(8i32 as (usize));
    let max_commands_per_metablock
        : usize
        = max_metablock_size.wrapping_div(8i32 as (usize));
    let mut metablock_start : usize = 0i32 as (usize);
    let mut prev_byte : u8 = 0i32 as (u8);
    let mut prev_byte2 : u8 = 0i32 as (u8);
    BrotliEncoderInitParams(&mut params as (*mut BrotliEncoderParams));
    params.quality = 10i32;
    params.lgwin = lgwin;
    SanitizeParams(&mut params as (*mut BrotliEncoderParams));
    params.lgblock = ComputeLgBlock(
                         &mut params as (*mut BrotliEncoderParams) as (*const BrotliEncoderParams)
                     );
    max_block_size = 1i32 as (usize) << params.lgblock;
    BrotliInitMemoryManager(
        m,
        0i32 as (unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void),
        0i32 as (unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void)),
        0i32 as (*mut ::std::os::raw::c_void)
    );
    0i32;
    EncodeWindowBits(
        lgwin,
        &mut last_byte as (*mut u8),
        &mut last_byte_bits as (*mut u8)
    );
    InitOrStitchToPreviousBlock(
        m,
        &mut hasher as (*mut *mut u8),
        input_buffer,
        mask,
        &mut params as (*mut BrotliEncoderParams),
        0i32 as (usize),
        hasher_eff_size,
        1i32
    );
    if !(0i32 == 0) {
        BrotliWipeOutMemoryManager(m);
        return 0i32;
    }
    while ok != 0 && (metablock_start < input_size) {
        let metablock_end
            : usize
            = brotli_min_size_t(
                  input_size,
                  metablock_start.wrapping_add(max_metablock_size)
              );
        let expected_num_commands
            : usize
            = metablock_end.wrapping_sub(metablock_start).wrapping_div(
                  12i32 as (usize)
              ).wrapping_add(
                  16i32 as (usize)
              );
        let mut commands : *mut Command = 0i32 as (*mut Command);
        let mut num_commands : usize = 0i32 as (usize);
        let mut last_insert_len : usize = 0i32 as (usize);
        let mut num_literals : usize = 0i32 as (usize);
        let mut metablock_size : usize = 0i32 as (usize);
        let mut cmd_alloc_size : usize = 0i32 as (usize);
        let mut is_last : i32;
        let mut storage : *mut u8;
        let mut storage_ix : usize;
        let mut block_start : usize;
        block_start = metablock_start;
        while block_start < metablock_end {
            let mut block_size
                : usize
                = brotli_min_size_t(
                      metablock_end.wrapping_sub(block_start),
                      max_block_size
                  );
            let mut nodes
                : *mut ZopfliNode
                = if block_size.wrapping_add(1i32 as (usize)) != 0 {
                      BrotliAllocate(
                          m,
                          block_size.wrapping_add(1i32 as (usize)).wrapping_mul(
                              ::std::mem::size_of::<ZopfliNode>()
                          )
                      ) as (*mut ZopfliNode)
                  } else {
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut ZopfliNode)
                  };
            let mut path_size : usize;
            let mut new_cmd_alloc_size : usize;
            if !(0i32 == 0) {
                BrotliWipeOutMemoryManager(m);
                return 0i32;
            }
            BrotliInitZopfliNodes(
                nodes,
                block_size.wrapping_add(1i32 as (usize))
            );
            StitchToPreviousBlockH10(
                hasher,
                block_size,
                block_start,
                input_buffer,
                mask
            );
            path_size = BrotliZopfliComputeShortestPath(
                            m,
                            dictionary,
                            block_size,
                            block_start,
                            input_buffer,
                            mask,
                            &mut params as (*mut BrotliEncoderParams) as (*const BrotliEncoderParams),
                            max_backward_limit,
                            dist_cache.as_mut_ptr() as (*const i32),
                            hasher,
                            nodes
                        );
            if !(0i32 == 0) {
                BrotliWipeOutMemoryManager(m);
                return 0i32;
            }
            new_cmd_alloc_size = brotli_max_size_t(
                                     expected_num_commands,
                                     num_commands.wrapping_add(path_size).wrapping_add(
                                         1i32 as (usize)
                                     )
                                 );
            if cmd_alloc_size != new_cmd_alloc_size {
                let mut new_commands
                    : *mut Command
                    = if new_cmd_alloc_size != 0 {
                          BrotliAllocate(
                              m,
                              new_cmd_alloc_size.wrapping_mul(::std::mem::size_of::<Command>())
                          ) as (*mut Command)
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut Command)
                      };
                if !(0i32 == 0) {
                    BrotliWipeOutMemoryManager(m);
                    return 0i32;
                }
                cmd_alloc_size = new_cmd_alloc_size;
                if !commands.is_null() {
                    memcpy(
                        new_commands as (*mut ::std::os::raw::c_void),
                        commands as (*const ::std::os::raw::c_void),
                        ::std::mem::size_of::<Command>().wrapping_mul(num_commands)
                    );
                    {
                        BrotliFree(m,commands as (*mut ::std::os::raw::c_void));
                        commands = 0i32 as (*mut ::std::os::raw::c_void) as (*mut Command);
                    }
                }
                commands = new_commands;
            }
            BrotliZopfliCreateCommands(
                block_size,
                block_start,
                max_backward_limit,
                &mut *nodes.offset(
                          0i32 as (isize)
                      ) as (*mut ZopfliNode) as (*const ZopfliNode),
                dist_cache.as_mut_ptr(),
                &mut last_insert_len as (*mut usize),
                &mut *commands.offset(num_commands as (isize)) as (*mut Command),
                &mut num_literals as (*mut usize)
            );
            num_commands = num_commands.wrapping_add(path_size);
            block_start = block_start.wrapping_add(block_size);
            metablock_size = metablock_size.wrapping_add(block_size);
            {
                BrotliFree(m,nodes as (*mut ::std::os::raw::c_void));
                nodes = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ZopfliNode);
            }
            if num_literals > max_literals_per_metablock || num_commands > max_commands_per_metablock {
                if 1337i32 != 0 {
                    break;
                }
            }
        }
        if last_insert_len > 0i32 as (usize) {
            InitInsertCommand(
                &mut *commands.offset(
                          {
                              let _old = num_commands;
                              num_commands = num_commands.wrapping_add(1 as (usize));
                              _old
                          } as (isize)
                      ) as (*mut Command),
                last_insert_len
            );
            num_literals = num_literals.wrapping_add(last_insert_len);
        }
        is_last = if !!(metablock_start.wrapping_add(
                            metablock_size
                        ) == input_size) {
                      1i32
                  } else {
                      0i32
                  };
        storage = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        storage_ix = last_byte_bits as (usize);
        if metablock_size == 0i32 as (usize) {
            storage = if 16i32 != 0 {
                          BrotliAllocate(
                              m,
                              (16i32 as (usize)).wrapping_mul(::std::mem::size_of::<u8>())
                          ) as (*mut u8)
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                      };
            if !(0i32 == 0) {
                BrotliWipeOutMemoryManager(m);
                return 0i32;
            }
            *storage.offset(0i32 as (isize)) = last_byte;
            BrotliWriteBits(
                2i32 as (usize),
                3i32 as (usize),
                &mut storage_ix as (*mut usize),
                storage
            );
            storage_ix = storage_ix.wrapping_add(
                             7u32 as (usize)
                         ) & !7u32 as (usize);
        } else if ShouldCompress(
                      input_buffer,
                      mask,
                      metablock_start,
                      metablock_size,
                      num_literals,
                      num_commands
                  ) == 0 {
            memcpy(
                dist_cache.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                saved_dist_cache.as_mut_ptr() as (*const ::std::os::raw::c_void),
                (4i32 as (usize)).wrapping_mul(::std::mem::size_of::<i32>())
            );
            storage = if metablock_size.wrapping_add(16i32 as (usize)) != 0 {
                          BrotliAllocate(
                              m,
                              metablock_size.wrapping_add(16i32 as (usize)).wrapping_mul(
                                  ::std::mem::size_of::<u8>()
                              )
                          ) as (*mut u8)
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                      };
            if !(0i32 == 0) {
                BrotliWipeOutMemoryManager(m);
                return 0i32;
            }
            *storage.offset(0i32 as (isize)) = last_byte;
            BrotliStoreUncompressedMetaBlock(
                is_last,
                input_buffer,
                metablock_start,
                mask,
                metablock_size,
                &mut storage_ix as (*mut usize),
                storage
            );
        } else {
            let mut num_direct_distance_codes : u32 = 0i32 as (u32);
            let mut distance_postfix_bits : u32 = 0i32 as (u32);
            let mut literal_context_mode
                : ContextType
                = ContextType::CONTEXT_UTF8;
            let mut mb : MetaBlockSplit;
            InitMetaBlockSplit(&mut mb as (*mut MetaBlockSplit));
            if BrotliIsMostlyUTF8(
                   input_buffer,
                   metablock_start,
                   mask,
                   metablock_size,
                   kMinUTF8Ratio
               ) == 0 {
                literal_context_mode = ContextType::CONTEXT_SIGNED;
            }
            BrotliBuildMetaBlock(
                m,
                input_buffer,
                metablock_start,
                mask,
                &mut params as (*mut BrotliEncoderParams) as (*const BrotliEncoderParams),
                prev_byte,
                prev_byte2,
                commands as (*const Command),
                num_commands,
                literal_context_mode,
                &mut mb as (*mut MetaBlockSplit)
            );
            if !(0i32 == 0) {
                BrotliWipeOutMemoryManager(m);
                return 0i32;
            }
            BrotliOptimizeHistograms(
                num_direct_distance_codes as (usize),
                distance_postfix_bits as (usize),
                &mut mb as (*mut MetaBlockSplit)
            );
            storage = if (2i32 as (usize)).wrapping_mul(
                             metablock_size
                         ).wrapping_add(
                             502i32 as (usize)
                         ) != 0 {
                          BrotliAllocate(
                              m,
                              (2i32 as (usize)).wrapping_mul(metablock_size).wrapping_add(
                                  502i32 as (usize)
                              ).wrapping_mul(
                                  ::std::mem::size_of::<u8>()
                              )
                          ) as (*mut u8)
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                      };
            if !(0i32 == 0) {
                BrotliWipeOutMemoryManager(m);
                return 0i32;
            }
            *storage.offset(0i32 as (isize)) = last_byte;
            BrotliStoreMetaBlock(
                m,
                input_buffer,
                metablock_start,
                metablock_size,
                mask,
                prev_byte,
                prev_byte2,
                is_last,
                num_direct_distance_codes,
                distance_postfix_bits,
                literal_context_mode,
                commands as (*const Command),
                num_commands,
                &mut mb as (*mut MetaBlockSplit) as (*const MetaBlockSplit),
                &mut storage_ix as (*mut usize),
                storage
            );
            if !(0i32 == 0) {
                BrotliWipeOutMemoryManager(m);
                return 0i32;
            }
            if metablock_size.wrapping_add(
                   4i32 as (usize)
               ) < storage_ix >> 3i32 {
                memcpy(
                    dist_cache.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                    saved_dist_cache.as_mut_ptr() as (*const ::std::os::raw::c_void),
                    (4i32 as (usize)).wrapping_mul(::std::mem::size_of::<i32>())
                );
                *storage.offset(0i32 as (isize)) = last_byte;
                storage_ix = last_byte_bits as (usize);
                BrotliStoreUncompressedMetaBlock(
                    is_last,
                    input_buffer,
                    metablock_start,
                    mask,
                    metablock_size,
                    &mut storage_ix as (*mut usize),
                    storage
                );
            }
            DestroyMetaBlockSplit(m,&mut mb as (*mut MetaBlockSplit));
        }
        last_byte = *storage.offset((storage_ix >> 3i32) as (isize));
        last_byte_bits = (storage_ix & 7u32 as (usize)) as (u8);
        metablock_start = metablock_start.wrapping_add(metablock_size);
        prev_byte = *input_buffer.offset(
                         metablock_start.wrapping_sub(1i32 as (usize)) as (isize)
                     );
        prev_byte2 = *input_buffer.offset(
                          metablock_start.wrapping_sub(2i32 as (usize)) as (isize)
                      );
        memcpy(
            saved_dist_cache.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            dist_cache.as_mut_ptr() as (*const ::std::os::raw::c_void),
            (4i32 as (usize)).wrapping_mul(::std::mem::size_of::<i32>())
        );
        {
            let out_size : usize = storage_ix >> 3i32;
            total_out_size = total_out_size.wrapping_add(out_size);
            if total_out_size <= max_out_size {
                memcpy(
                    encoded_buffer as (*mut ::std::os::raw::c_void),
                    storage as (*const ::std::os::raw::c_void),
                    out_size
                );
                encoded_buffer = encoded_buffer.offset(out_size as (isize));
            } else {
                ok = 0i32;
            }
        }
        {
            BrotliFree(m,storage as (*mut ::std::os::raw::c_void));
            storage = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        {
            BrotliFree(m,commands as (*mut ::std::os::raw::c_void));
            commands = 0i32 as (*mut ::std::os::raw::c_void) as (*mut Command);
        }
    }
    *encoded_size = total_out_size;
    DestroyHasher(m,&mut hasher as (*mut *mut u8));
    return ok;
    BrotliWipeOutMemoryManager(m);
    0i32
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum BrotliEncoderOperation {
    BROTLI_OPERATION_PROCESS = 0i32,
    BROTLI_OPERATION_FLUSH = 1i32,
    BROTLI_OPERATION_FINISH = 2i32,
    BROTLI_OPERATION_EMIT_METADATA = 3i32,
}

unsafe extern fn MakeUncompressedStream(
    mut input : *const u8, mut input_size : usize, mut output : *mut u8
) -> usize {
    let mut size : usize = input_size;
    let mut result : usize = 0i32 as (usize);
    let mut offset : usize = 0i32 as (usize);
    if input_size == 0i32 as (usize) {
        *output.offset(0i32 as (isize)) = 6i32 as (u8);
        return 1i32 as (usize);
    }
    *output.offset(
         {
             let _old = result;
             result = result.wrapping_add(1 as (usize));
             _old
         } as (isize)
     ) = 0x21i32 as (u8);
    *output.offset(
         {
             let _old = result;
             result = result.wrapping_add(1 as (usize));
             _old
         } as (isize)
     ) = 0x3i32 as (u8);
    while size > 0i32 as (usize) {
        let mut nibbles : u32 = 0i32 as (u32);
        let mut chunk_size : u32;
        let mut bits : u32;
        chunk_size = if size > (1u32 << 24i32) as (usize) {
                         1u32 << 24i32
                     } else {
                         size as (u32)
                     };
        if chunk_size > 1u32 << 16i32 {
            nibbles = if chunk_size > 1u32 << 20i32 {
                          2i32
                      } else {
                          1i32
                      } as (u32);
        }
        bits = nibbles << 1i32 | chunk_size.wrapping_sub(
                                     1i32 as (u32)
                                 ) << 3i32 | 1u32 << (19i32 as (u32)).wrapping_add(
                                                         (4i32 as (u32)).wrapping_mul(nibbles)
                                                     );
        *output.offset(
             {
                 let _old = result;
                 result = result.wrapping_add(1 as (usize));
                 _old
             } as (isize)
         ) = bits as (u8);
        *output.offset(
             {
                 let _old = result;
                 result = result.wrapping_add(1 as (usize));
                 _old
             } as (isize)
         ) = (bits >> 8i32) as (u8);
        *output.offset(
             {
                 let _old = result;
                 result = result.wrapping_add(1 as (usize));
                 _old
             } as (isize)
         ) = (bits >> 16i32) as (u8);
        if nibbles == 2i32 as (u32) {
            *output.offset(
                 {
                     let _old = result;
                     result = result.wrapping_add(1 as (usize));
                     _old
                 } as (isize)
             ) = (bits >> 24i32) as (u8);
        }
        memcpy(
            &mut *output.offset(
                      result as (isize)
                  ) as (*mut u8) as (*mut ::std::os::raw::c_void),
            &*input.offset(
                  offset as (isize)
              ) as (*const u8) as (*const ::std::os::raw::c_void),
            chunk_size as (usize)
        );
        result = result.wrapping_add(chunk_size as (usize));
        offset = offset.wrapping_add(chunk_size as (usize));
        size = size.wrapping_sub(chunk_size as (usize));
    }
    *output.offset(
         {
             let _old = result;
             result = result.wrapping_add(1 as (usize));
             _old
         } as (isize)
     ) = 3i32 as (u8);
    result
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderCompress(
    mut quality : i32,
    mut lgwin : i32,
    mut mode : BrotliEncoderMode,
    mut input_size : usize,
    mut input_buffer : *const u8,
    mut encoded_size : *mut usize,
    mut encoded_buffer : *mut u8
) -> i32 {
    let mut s : *mut BrotliEncoderStateStruct;
    let mut out_size : usize = *encoded_size;
    let mut input_start : *const u8 = input_buffer;
    let mut output_start : *mut u8 = encoded_buffer;
    let mut max_out_size
        : usize
        = BrotliEncoderMaxCompressedSize(input_size);
    if out_size == 0i32 as (usize) {
        return 0i32;
    }
    if input_size == 0i32 as (usize) {
        *encoded_size = 1i32 as (usize);
        *encoded_buffer = 6i32 as (u8);
        return 1i32;
    }
    let mut is_fallback : i32 = 0i32;
    if quality == 10i32 {
        let lg_win
            : i32
            = brotli_min_int(24i32,brotli_max_int(16i32,lgwin));
        let mut ok
            : i32
            = BrotliCompressBufferQuality10(
                  lg_win,
                  input_size,
                  input_buffer,
                  encoded_size,
                  encoded_buffer
              );
        if ok == 0 || max_out_size != 0 && (*encoded_size > max_out_size) {
            is_fallback = 1i32;
        } else {
            return 1i32;
        }
    }
    if is_fallback == 0 {
        s = BrotliEncoderCreateInstance(
                0i32 as (unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void),
                0i32 as (unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void)),
                0i32 as (*mut ::std::os::raw::c_void)
            );
        if s.is_null() {
            return 0i32;
        } else {
            let mut available_in : usize = input_size;
            let mut next_in : *const u8 = input_buffer;
            let mut available_out : usize = *encoded_size;
            let mut next_out : *mut u8 = encoded_buffer;
            let mut total_out : usize = 0i32 as (usize);
            let mut result : i32 = 0i32;
            BrotliEncoderSetParameter(
                s,
                BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
                quality as (u32)
            );
            BrotliEncoderSetParameter(
                s,
                BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
                lgwin as (u32)
            );
            BrotliEncoderSetParameter(
                s,
                BrotliEncoderParameter::BROTLI_PARAM_MODE,
                mode as (u32)
            );
            BrotliEncoderSetParameter(
                s,
                BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT,
                input_size as (u32)
            );
            result = BrotliEncoderCompressStream(
                         s,
                         BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
                         &mut available_in as (*mut usize),
                         &mut next_in as (*mut *const u8),
                         &mut available_out as (*mut usize),
                         &mut next_out as (*mut *mut u8),
                         &mut total_out as (*mut usize)
                     );
            if BrotliEncoderIsFinished(s) == 0 {
                result = 0i32;
            }
            *encoded_size = total_out;
            BrotliEncoderDestroyInstance(s);
            if result == 0 || max_out_size != 0 && (*encoded_size > max_out_size) {
                is_fallback = 1i32;
            } else {
                return 1i32;
            }
        }
    }
    *encoded_size = 0i32 as (usize);
    if max_out_size == 0 {
        return 0i32;
    }
    if out_size >= max_out_size {
        *encoded_size = MakeUncompressedStream(
                            input_start,
                            input_size,
                            output_start
                        );
        return 1i32;
    }
    0i32
}

unsafe extern fn UnprocessedInputSize(
    mut s : *mut BrotliEncoderStateStruct
) -> usize {
    (*s).input_pos_.wrapping_sub((*s).last_processed_pos_)
}

unsafe extern fn UpdateSizeHint(
    mut s : *mut BrotliEncoderStateStruct, mut available_in : usize
) { if (*s).params.size_hint == 0i32 as (usize) {
        let mut delta : usize = UnprocessedInputSize(s);
        let mut tail : usize = available_in;
        let mut limit : u32 = 1u32 << 30i32;
        let mut total : u32;
        if delta >= limit as (usize) || tail >= limit as (usize) || delta.wrapping_add(
                                                                        tail
                                                                    ) >= limit as (usize) {
            total = limit;
        } else {
            total = delta.wrapping_add(tail) as (u32);
        }
        (*s).params.size_hint = total as (usize);
    }
}

unsafe extern fn InjectBytePaddingBlock(
    mut s : *mut BrotliEncoderStateStruct
) {
    let mut seal : u32 = (*s).last_byte_ as (u32);
    let mut seal_bits : usize = (*s).last_byte_bits_ as (usize);
    let mut destination : *mut u8;
    (*s).last_byte_ = 0i32 as (u8);
    (*s).last_byte_bits_ = 0i32 as (u8);
    seal = seal | 0x6u32 << seal_bits;
    seal_bits = seal_bits.wrapping_add(6i32 as (usize));
    if !(*s).next_out_.is_null() {
        destination = (*s).next_out_.offset(
                          (*s).available_out_ as (isize)
                      );
    } else {
        destination = (*s).tiny_buf_.u8.as_mut_ptr();
        (*s).next_out_ = destination;
    }
    *destination.offset(0i32 as (isize)) = seal as (u8);
    if seal_bits > 8i32 as (usize) {
        *destination.offset(1i32 as (isize)) = (seal >> 8i32) as (u8);
    }
    (*s).available_out_ = (*s).available_out_.wrapping_add(
                              seal_bits.wrapping_add(7i32 as (usize)) >> 3i32
                          );
}

unsafe extern fn InjectFlushOrPushOutput(
    mut s : *mut BrotliEncoderStateStruct,
    mut available_out : *mut usize,
    mut next_out : *mut *mut u8,
    mut total_out : *mut usize
) -> i32 {
    if (*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED as (i32) && ((*s).last_byte_bits_ as (i32) != 0i32) {
        InjectBytePaddingBlock(s);
        return 1i32;
    }
    if (*s).available_out_ != 0i32 as (usize) && (*available_out != 0i32 as (usize)) {
        let mut copy_output_size
            : usize
            = brotli_min_size_t((*s).available_out_,*available_out);
        memcpy(
            *next_out as (*mut ::std::os::raw::c_void),
            (*s).next_out_ as (*const ::std::os::raw::c_void),
            copy_output_size
        );
        *next_out = (*next_out).offset(copy_output_size as (isize));
        *available_out = (*available_out).wrapping_sub(copy_output_size);
        (*s).next_out_ = (*s).next_out_.offset(
                             copy_output_size as (isize)
                         );
        (*s).available_out_ = (*s).available_out_.wrapping_sub(
                                  copy_output_size
                              );
        (*s).total_out_ = (*s).total_out_.wrapping_add(copy_output_size);
        if !total_out.is_null() {
            *total_out = (*s).total_out_;
        }
        return 1i32;
    }
    0i32
}

unsafe extern fn WrapPosition(mut position : usize) -> u32 {
    let mut result : u32 = position as (u32);
    let mut gb : usize = position >> 30i32;
    if gb > 2i32 as (usize) {
        result = result & (1u32 << 30i32).wrapping_sub(
                              1i32 as (u32)
                          ) | ((gb.wrapping_sub(
                                    1i32 as (usize)
                                ) & 1i32 as (usize)) as (u32)).wrapping_add(
                                  1i32 as (u32)
                              ) << 30i32;
    }
    result
}

unsafe extern fn InputBlockSize(
    mut s : *mut BrotliEncoderStateStruct
) -> usize {
    if EnsureInitialized(s) == 0 {
        return 0i32 as (usize);
    }
    1i32 as (usize) << (*s).params.lgblock
}

unsafe extern fn GetBrotliStorage(
    mut s : *mut BrotliEncoderStateStruct, mut size : usize
) -> *mut u8 {
    let mut m
        : *mut MemoryManager
        = &mut (*s).memory_manager_ as (*mut MemoryManager);
    if (*s).storage_size_ < size {
        {
            BrotliFree(m,(*s).storage_ as (*mut ::std::os::raw::c_void));
            (*s).storage_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        (*s).storage_ = if size != 0 {
                            BrotliAllocate(
                                m,
                                size.wrapping_mul(::std::mem::size_of::<u8>())
                            ) as (*mut u8)
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                        };
        if !(0i32 == 0) {
            return 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        (*s).storage_size_ = size;
    }
    (*s).storage_
}

unsafe extern fn MaxHashTableSize(mut quality : i32) -> usize {
    (if quality == 0i32 {
         1i32 << 15i32
     } else {
         1i32 << 17i32
     }) as (usize)
}

unsafe extern fn HashTableSize(
    mut max_table_size : usize, mut input_size : usize
) -> usize {
    let mut htsize : usize = 256i32 as (usize);
    while htsize < max_table_size && (htsize < input_size) {
        htsize = htsize << 1i32;
    }
    htsize
}

unsafe extern fn GetHashTable(
    mut s : *mut BrotliEncoderStateStruct,
    mut quality : i32,
    mut input_size : usize,
    mut table_size : *mut usize
) -> *mut i32 {
    let mut m
        : *mut MemoryManager
        = &mut (*s).memory_manager_ as (*mut MemoryManager);
    let max_table_size : usize = MaxHashTableSize(quality);
    let mut htsize : usize = HashTableSize(max_table_size,input_size);
    let mut table : *mut i32;
    0i32;
    if quality == 0i32 {
        if htsize & 0xaaaaai32 as (usize) == 0i32 as (usize) {
            htsize = htsize << 1i32;
        }
    }
    if htsize <= ::std::mem::size_of::<[i32; 1024]>().wrapping_div(
                     ::std::mem::size_of::<i32>()
                 ) {
        table = (*s).small_table_.as_mut_ptr();
    } else {
        if htsize > (*s).large_table_size_ {
            (*s).large_table_size_ = htsize;
            {
                BrotliFree(m,(*s).large_table_ as (*mut ::std::os::raw::c_void));
                (*s).large_table_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut i32);
            }
            (*s).large_table_ = if htsize != 0 {
                                    BrotliAllocate(
                                        m,
                                        htsize.wrapping_mul(::std::mem::size_of::<i32>())
                                    ) as (*mut i32)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut i32)
                                };
            if !(0i32 == 0) {
                return 0i32 as (*mut i32);
            }
        }
        table = (*s).large_table_;
    }
    *table_size = htsize;
    memset(
        table as (*mut ::std::os::raw::c_void),
        0i32,
        htsize.wrapping_mul(::std::mem::size_of::<i32>())
    );
    table
}

unsafe extern fn UpdateLastProcessedPos(
    mut s : *mut BrotliEncoderStateStruct
) -> i32 {
    let mut wrapped_last_processed_pos
        : u32
        = WrapPosition((*s).last_processed_pos_);
    let mut wrapped_input_pos : u32 = WrapPosition((*s).input_pos_);
    (*s).last_processed_pos_ = (*s).input_pos_;
    if !!(wrapped_input_pos < wrapped_last_processed_pos) {
        1i32
    } else {
        0i32
    }
}

unsafe extern fn MaxMetablockSize(
    mut params : *const BrotliEncoderParams
) -> usize {
    let mut bits : i32 = brotli_min_int(ComputeRbBits(params),24i32);
    1i32 as (usize) << bits
}

unsafe extern fn CommandCopyLen(mut self : *const Command) -> u32 {
    (*self).copy_len_ & 0xffffffi32 as (u32)
}

unsafe extern fn PrefixEncodeCopyDistance(
    mut distance_code : usize,
    mut num_direct_codes : usize,
    mut postfix_bits : usize,
    mut code : *mut u16,
    mut extra_bits : *mut u32
) { if distance_code < (16i32 as (usize)).wrapping_add(
                           num_direct_codes
                       ) {
        *code = distance_code as (u16);
        *extra_bits = 0i32 as (u32);
    } else {
        let mut dist
            : usize
            = (1i32 as (usize) << postfix_bits.wrapping_add(
                                      2u32 as (usize)
                                  )).wrapping_add(
                  distance_code.wrapping_sub(16i32 as (usize)).wrapping_sub(
                      num_direct_codes
                  )
              );
        let mut bucket
            : usize
            = Log2FloorNonZero(dist).wrapping_sub(1i32 as (u32)) as (usize);
        let mut postfix_mask
            : usize
            = (1u32 << postfix_bits).wrapping_sub(1i32 as (u32)) as (usize);
        let mut postfix : usize = dist & postfix_mask;
        let mut prefix : usize = dist >> bucket & 1i32 as (usize);
        let mut offset
            : usize
            = (2i32 as (usize)).wrapping_add(prefix) << bucket;
        let mut nbits : usize = bucket.wrapping_sub(postfix_bits);
        *code = (16i32 as (usize)).wrapping_add(
                    num_direct_codes
                ).wrapping_add(
                    (2i32 as (usize)).wrapping_mul(
                        nbits.wrapping_sub(1i32 as (usize))
                    ).wrapping_add(
                        prefix
                    ) << postfix_bits
                ).wrapping_add(
                    postfix
                ) as (u16);
        *extra_bits = (nbits << 24i32 | dist.wrapping_sub(
                                            offset
                                        ) >> postfix_bits) as (u32);
    }
}

unsafe extern fn CommandRestoreDistanceCode(
    mut self : *const Command
) -> u32 {
    if (*self).dist_prefix_ as (i32) < 16i32 {
        (*self).dist_prefix_ as (u32)
    } else {
        let mut nbits : u32 = (*self).dist_extra_ >> 24i32;
        let mut extra : u32 = (*self).dist_extra_ & 0xffffffi32 as (u32);
        let mut prefix
            : u32
            = ((*self).dist_prefix_ as (u32)).wrapping_add(4u32).wrapping_sub(
                  16i32 as (u32)
              ).wrapping_sub(
                  2u32.wrapping_mul(nbits)
              );
        (prefix << nbits).wrapping_add(extra).wrapping_add(
            16i32 as (u32)
        ).wrapping_sub(
            4u32
        )
    }
}

unsafe extern fn RecomputeDistancePrefixes(
    mut cmds : *mut Command,
    mut num_commands : usize,
    mut num_direct_distance_codes : u32,
    mut distance_postfix_bits : u32
) {
    let mut i : usize;
    if num_direct_distance_codes == 0i32 as (u32) && (distance_postfix_bits == 0i32 as (u32)) {
        return;
    }
    i = 0i32 as (usize);
    while i < num_commands {
        {
            let mut cmd
                : *mut Command
                = &mut *cmds.offset(i as (isize)) as (*mut Command);
            if CommandCopyLen(
                   cmd as (*const Command)
               ) != 0 && ((*cmd).cmd_prefix_ as (i32) >= 128i32) {
                PrefixEncodeCopyDistance(
                    CommandRestoreDistanceCode(cmd as (*const Command)) as (usize),
                    num_direct_distance_codes as (usize),
                    distance_postfix_bits as (usize),
                    &mut (*cmd).dist_prefix_ as (*mut u16),
                    &mut (*cmd).dist_extra_ as (*mut u32)
                );
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn ChooseContextMap(
    mut quality : i32,
    mut bigram_histo : *mut u32,
    mut num_literal_contexts : *mut usize,
    mut literal_context_map : *mut *const u32
) {
    static mut kStaticContextMapContinuation
        : [u32; 64]
        = [   1i32 as (u32),
              1i32 as (u32),
              2i32 as (u32),
              2i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
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
    static mut kStaticContextMapSimpleUTF8
        : [u32; 64]
        = [   0i32 as (u32),
              0i32 as (u32),
              1i32 as (u32),
              1i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
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
    let mut monogram_histo
        : [u32; 3]
        = [ 0i32 as (u32), 0i32 as (u32), 0i32 as (u32) ];
    let mut two_prefix_histo
        : [u32; 6]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut total : usize;
    let mut i : usize;
    let mut dummy : usize;
    let mut entropy : [f64; 4];
    i = 0i32 as (usize);
    while i < 9i32 as (usize) {
        {
            {
                let _rhs = *bigram_histo.offset(i as (isize));
                let _lhs = &mut monogram_histo[i.wrapping_rem(3i32 as (usize))];
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            {
                let _rhs = *bigram_histo.offset(i as (isize));
                let _lhs = &mut two_prefix_histo[i.wrapping_rem(6i32 as (usize))];
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    entropy[1i32 as (usize)] = ShannonEntropy(
                                   monogram_histo.as_mut_ptr() as (*const u32),
                                   3i32 as (usize),
                                   &mut dummy as (*mut usize)
                               );
    entropy[2i32 as (usize)] = ShannonEntropy(
                                   two_prefix_histo.as_mut_ptr() as (*const u32),
                                   3i32 as (usize),
                                   &mut dummy as (*mut usize)
                               ) + ShannonEntropy(
                                       two_prefix_histo.as_mut_ptr().offset(
                                           3i32 as (isize)
                                       ) as (*const u32),
                                       3i32 as (usize),
                                       &mut dummy as (*mut usize)
                                   );
    entropy[3i32 as (usize)] = 0i32 as (f64);
    i = 0i32 as (usize);
    while i < 3i32 as (usize) {
        {
            let _rhs
                = ShannonEntropy(
                      bigram_histo.offset(
                          (3i32 as (usize)).wrapping_mul(i) as (isize)
                      ) as (*const u32),
                      3i32 as (usize),
                      &mut dummy as (*mut usize)
                  );
            let _lhs = &mut entropy[3i32 as (usize)];
            *_lhs = *_lhs + _rhs;
        }
        i = i.wrapping_add(1 as (usize));
    }
    total = monogram_histo[0i32 as (usize)].wrapping_add(
                monogram_histo[1i32 as (usize)]
            ).wrapping_add(
                monogram_histo[2i32 as (usize)]
            ) as (usize);
    0i32;
    entropy[0i32 as (usize)] = 1.0f64 / total as (f64);
    {
        let _rhs = entropy[0i32 as (usize)];
        let _lhs = &mut entropy[1i32 as (usize)];
        *_lhs = *_lhs * _rhs;
    }
    {
        let _rhs = entropy[0i32 as (usize)];
        let _lhs = &mut entropy[2i32 as (usize)];
        *_lhs = *_lhs * _rhs;
    }
    {
        let _rhs = entropy[0i32 as (usize)];
        let _lhs = &mut entropy[3i32 as (usize)];
        *_lhs = *_lhs * _rhs;
    }
    if quality < 7i32 {
        entropy[3i32 as (usize)] = entropy[
                                       1i32 as (usize)
                                   ] * 10i32 as (f64);
    }
    if entropy[1i32 as (usize)] - entropy[
                                      2i32 as (usize)
                                  ] < 0.2f64 && (entropy[1i32 as (usize)] - entropy[
                                                                                3i32 as (usize)
                                                                            ] < 0.2f64) {
        *num_literal_contexts = 1i32 as (usize);
    } else if entropy[2i32 as (usize)] - entropy[
                                             3i32 as (usize)
                                         ] < 0.02f64 {
        *num_literal_contexts = 2i32 as (usize);
        *literal_context_map = kStaticContextMapSimpleUTF8.as_ptr();
    } else {
        *num_literal_contexts = 3i32 as (usize);
        *literal_context_map = kStaticContextMapContinuation.as_ptr();
    }
}

unsafe extern fn DecideOverLiteralContextModeling(
    mut input : *const u8,
    mut start_pos : usize,
    mut length : usize,
    mut mask : usize,
    mut quality : i32,
    mut literal_context_mode : *mut ContextType,
    mut num_literal_contexts : *mut usize,
    mut literal_context_map : *mut *const u32
) { if quality < 5i32 || length < 64i32 as (usize) {
    } else {
        let end_pos : usize = start_pos.wrapping_add(length);
        let mut bigram_prefix_histo
            : [u32; 9]
            = [   0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32)
              ];
        while start_pos.wrapping_add(64i32 as (usize)) <= end_pos {
            {
                static mut lut : [i32; 4] = [ 0i32, 0i32, 1i32, 2i32 ];
                let stride_end_pos
                    : usize
                    = start_pos.wrapping_add(64i32 as (usize));
                let mut prev
                    : i32
                    = lut[
                          (*input.offset(
                                (start_pos & mask) as (isize)
                            ) as (i32) >> 6i32) as (usize)
                      ] * 3i32;
                let mut pos : usize;
                pos = start_pos.wrapping_add(1i32 as (usize));
                while pos < stride_end_pos {
                    {
                        let literal : u8 = *input.offset((pos & mask) as (isize));
                        {
                            let _rhs = 1;
                            let _lhs
                                = &mut bigram_prefix_histo[
                                           (prev + lut[
                                                       (literal as (i32) >> 6i32) as (usize)
                                                   ]) as (usize)
                                       ];
                            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                        }
                        prev = lut[(literal as (i32) >> 6i32) as (usize)] * 3i32;
                    }
                    pos = pos.wrapping_add(1 as (usize));
                }
            }
            start_pos = start_pos.wrapping_add(4096i32 as (usize));
        }
        *literal_context_mode = ContextType::CONTEXT_UTF8;
        ChooseContextMap(
            quality,
            &mut bigram_prefix_histo[0i32 as (usize)] as (*mut u32),
            num_literal_contexts,
            literal_context_map
        );
    }
}

unsafe extern fn WriteMetaBlockInternal(
    mut m : *mut MemoryManager,
    mut data : *const u8,
    mask : usize,
    last_flush_pos : usize,
    bytes : usize,
    is_last : i32,
    mut params : *const BrotliEncoderParams,
    prev_byte : u8,
    prev_byte2 : u8,
    num_literals : usize,
    num_commands : usize,
    mut commands : *mut Command,
    mut saved_dist_cache : *const i32,
    mut dist_cache : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let wrapped_last_flush_pos : u32 = WrapPosition(last_flush_pos);
    let mut last_byte : u8;
    let mut last_byte_bits : u8;
    let mut num_direct_distance_codes : u32 = 0i32 as (u32);
    let mut distance_postfix_bits : u32 = 0i32 as (u32);
    if bytes == 0i32 as (usize) {
        BrotliWriteBits(
            2i32 as (usize),
            3i32 as (usize),
            storage_ix,
            storage
        );
        *storage_ix = (*storage_ix).wrapping_add(
                          7u32 as (usize)
                      ) & !7u32 as (usize);
        return;
    }
    if ShouldCompress(
           data,
           mask,
           last_flush_pos,
           bytes,
           num_literals,
           num_commands
       ) == 0 {
        memcpy(
            dist_cache as (*mut ::std::os::raw::c_void),
            saved_dist_cache as (*const ::std::os::raw::c_void),
            (4i32 as (usize)).wrapping_mul(::std::mem::size_of::<i32>())
        );
        BrotliStoreUncompressedMetaBlock(
            is_last,
            data,
            wrapped_last_flush_pos as (usize),
            mask,
            bytes,
            storage_ix,
            storage
        );
        return;
    }
    last_byte = *storage.offset(0i32 as (isize));
    last_byte_bits = (*storage_ix & 0xffi32 as (usize)) as (u8);
    if (*params).quality >= 10i32 && ((*params).mode as (i32) == BrotliEncoderMode::BROTLI_MODE_FONT as (i32)) {
        num_direct_distance_codes = 12i32 as (u32);
        distance_postfix_bits = 1i32 as (u32);
        RecomputeDistancePrefixes(
            commands,
            num_commands,
            num_direct_distance_codes,
            distance_postfix_bits
        );
    }
    if (*params).quality <= 2i32 {
        BrotliStoreMetaBlockFast(
            m,
            data,
            wrapped_last_flush_pos as (usize),
            bytes,
            mask,
            is_last,
            commands as (*const Command),
            num_commands,
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
            return;
        }
    } else if (*params).quality < 4i32 {
        BrotliStoreMetaBlockTrivial(
            m,
            data,
            wrapped_last_flush_pos as (usize),
            bytes,
            mask,
            is_last,
            commands as (*const Command),
            num_commands,
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
            return;
        }
    } else {
        let mut literal_context_mode
            : ContextType
            = ContextType::CONTEXT_UTF8;
        let mut mb : MetaBlockSplit;
        InitMetaBlockSplit(&mut mb as (*mut MetaBlockSplit));
        if (*params).quality < 10i32 {
            let mut num_literal_contexts : usize = 1i32 as (usize);
            let mut literal_context_map
                : *const u32
                = 0i32 as (*mut ::std::os::raw::c_void) as (*const u32);
            if (*params).disable_literal_context_modeling == 0 {
                DecideOverLiteralContextModeling(
                    data,
                    wrapped_last_flush_pos as (usize),
                    bytes,
                    mask,
                    (*params).quality,
                    &mut literal_context_mode as (*mut ContextType),
                    &mut num_literal_contexts as (*mut usize),
                    &mut literal_context_map as (*mut *const u32)
                );
            }
            BrotliBuildMetaBlockGreedy(
                m,
                data,
                wrapped_last_flush_pos as (usize),
                mask,
                prev_byte,
                prev_byte2,
                literal_context_mode,
                num_literal_contexts,
                literal_context_map,
                commands as (*const Command),
                num_commands,
                &mut mb as (*mut MetaBlockSplit)
            );
            if !(0i32 == 0) {
                return;
            }
        } else {
            if BrotliIsMostlyUTF8(
                   data,
                   wrapped_last_flush_pos as (usize),
                   mask,
                   bytes,
                   kMinUTF8Ratio
               ) == 0 {
                literal_context_mode = ContextType::CONTEXT_SIGNED;
            }
            BrotliBuildMetaBlock(
                m,
                data,
                wrapped_last_flush_pos as (usize),
                mask,
                params,
                prev_byte,
                prev_byte2,
                commands as (*const Command),
                num_commands,
                literal_context_mode,
                &mut mb as (*mut MetaBlockSplit)
            );
            if !(0i32 == 0) {
                return;
            }
        }
        if (*params).quality >= 4i32 {
            BrotliOptimizeHistograms(
                num_direct_distance_codes as (usize),
                distance_postfix_bits as (usize),
                &mut mb as (*mut MetaBlockSplit)
            );
        }
        BrotliStoreMetaBlock(
            m,
            data,
            wrapped_last_flush_pos as (usize),
            bytes,
            mask,
            prev_byte,
            prev_byte2,
            is_last,
            num_direct_distance_codes,
            distance_postfix_bits,
            literal_context_mode,
            commands as (*const Command),
            num_commands,
            &mut mb as (*mut MetaBlockSplit) as (*const MetaBlockSplit),
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
            return;
        }
        DestroyMetaBlockSplit(m,&mut mb as (*mut MetaBlockSplit));
    }
    if bytes.wrapping_add(4i32 as (usize)) < *storage_ix >> 3i32 {
        memcpy(
            dist_cache as (*mut ::std::os::raw::c_void),
            saved_dist_cache as (*const ::std::os::raw::c_void),
            (4i32 as (usize)).wrapping_mul(::std::mem::size_of::<i32>())
        );
        *storage.offset(0i32 as (isize)) = last_byte;
        *storage_ix = last_byte_bits as (usize);
        BrotliStoreUncompressedMetaBlock(
            is_last,
            data,
            wrapped_last_flush_pos as (usize),
            mask,
            bytes,
            storage_ix,
            storage
        );
    }
}

unsafe extern fn EncodeData(
    mut s : *mut BrotliEncoderStateStruct,
    is_last : i32,
    force_flush : i32,
    mut out_size : *mut usize,
    mut output : *mut *mut u8
) -> i32 {
    let delta : usize = UnprocessedInputSize(s);
    let bytes : u32 = delta as (u32);
    let wrapped_last_processed_pos
        : u32
        = WrapPosition((*s).last_processed_pos_);
    let mut data : *mut u8;
    let mut mask : u32;
    let mut m
        : *mut MemoryManager
        = &mut (*s).memory_manager_ as (*mut MemoryManager);
    let mut dictionary
        : *const BrotliDictionary
        = BrotliGetDictionary();
    if EnsureInitialized(s) == 0 {
        return 0i32;
    }
    data = &mut *(*s).ringbuffer_.data_.offset(
                     (*s).ringbuffer_.buffer_index as (isize)
                 ) as (*mut u8);
    mask = (*s).ringbuffer_.mask_;
    if (*s).is_last_block_emitted_ != 0 {
        return 0i32;
    }
    if is_last != 0 {
        (*s).is_last_block_emitted_ = 1i32;
    }
    if delta > InputBlockSize(s) {
        return 0i32;
    }
    if (*s).params.quality == 1i32 && (*s).command_buf_.is_null() {
        (*s).command_buf_ = if kCompressFragmentTwoPassBlockSize != 0 {
                                BrotliAllocate(
                                    m,
                                    kCompressFragmentTwoPassBlockSize.wrapping_mul(
                                        ::std::mem::size_of::<u32>()
                                    )
                                ) as (*mut u32)
                            } else {
                                0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                            };
        (*s).literal_buf_ = if kCompressFragmentTwoPassBlockSize != 0 {
                                BrotliAllocate(
                                    m,
                                    kCompressFragmentTwoPassBlockSize.wrapping_mul(
                                        ::std::mem::size_of::<u8>()
                                    )
                                ) as (*mut u8)
                            } else {
                                0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                            };
        if !(0i32 == 0) {
            return 0i32;
        }
    }
    if (*s).params.quality == 0i32 || (*s).params.quality == 1i32 {
        let mut storage : *mut u8;
        let mut storage_ix : usize = (*s).last_byte_bits_ as (usize);
        let mut table_size : usize;
        let mut table : *mut i32;
        if delta == 0i32 as (usize) && (is_last == 0) {
            *out_size = 0i32 as (usize);
            return 1i32;
        }
        storage = GetBrotliStorage(
                      s,
                      (2i32 as (u32)).wrapping_mul(bytes).wrapping_add(
                          502i32 as (u32)
                      ) as (usize)
                  );
        if !(0i32 == 0) {
            return 0i32;
        }
        *storage.offset(0i32 as (isize)) = (*s).last_byte_;
        table = GetHashTable(
                    s,
                    (*s).params.quality,
                    bytes as (usize),
                    &mut table_size as (*mut usize)
                );
        if !(0i32 == 0) {
            return 0i32;
        }
        if (*s).params.quality == 0i32 {
            BrotliCompressFragmentFast(
                m,
                &mut *data.offset(
                          (wrapped_last_processed_pos & mask) as (isize)
                      ) as (*mut u8) as (*const u8),
                bytes as (usize),
                is_last,
                table,
                table_size,
                (*s).cmd_depths_.as_mut_ptr(),
                (*s).cmd_bits_.as_mut_ptr(),
                &mut (*s).cmd_code_numbits_ as (*mut usize),
                (*s).cmd_code_.as_mut_ptr(),
                &mut storage_ix as (*mut usize),
                storage
            );
            if !(0i32 == 0) {
                return 0i32;
            }
        } else {
            BrotliCompressFragmentTwoPass(
                m,
                &mut *data.offset(
                          (wrapped_last_processed_pos & mask) as (isize)
                      ) as (*mut u8) as (*const u8),
                bytes as (usize),
                is_last,
                (*s).command_buf_,
                (*s).literal_buf_,
                table,
                table_size,
                &mut storage_ix as (*mut usize),
                storage
            );
            if !(0i32 == 0) {
                return 0i32;
            }
        }
        (*s).last_byte_ = *storage.offset((storage_ix >> 3i32) as (isize));
        (*s).last_byte_bits_ = (storage_ix & 7u32 as (usize)) as (u8);
        UpdateLastProcessedPos(s);
        *output = &mut *storage.offset(0i32 as (isize)) as (*mut u8);
        *out_size = storage_ix >> 3i32;
        return 1i32;
    }
    {
        let mut newsize
            : usize
            = (*s).num_commands_.wrapping_add(
                  bytes.wrapping_div(2i32 as (u32)) as (usize)
              ).wrapping_add(
                  1i32 as (usize)
              );
        if newsize > (*s).cmd_alloc_size_ {
            let mut new_commands : *mut Command;
            newsize = newsize.wrapping_add(
                          bytes.wrapping_div(4i32 as (u32)).wrapping_add(
                              16i32 as (u32)
                          ) as (usize)
                      );
            (*s).cmd_alloc_size_ = newsize;
            new_commands = if newsize != 0 {
                               BrotliAllocate(
                                   m,
                                   newsize.wrapping_mul(::std::mem::size_of::<Command>())
                               ) as (*mut Command)
                           } else {
                               0i32 as (*mut ::std::os::raw::c_void) as (*mut Command)
                           };
            if !(0i32 == 0) {
                return 0i32;
            }
            if !(*s).commands_.is_null() {
                memcpy(
                    new_commands as (*mut ::std::os::raw::c_void),
                    (*s).commands_ as (*const ::std::os::raw::c_void),
                    ::std::mem::size_of::<Command>().wrapping_mul((*s).num_commands_)
                );
                {
                    BrotliFree(m,(*s).commands_ as (*mut ::std::os::raw::c_void));
                    (*s).commands_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut Command);
                }
            }
            (*s).commands_ = new_commands;
        }
    }
    InitOrStitchToPreviousBlock(
        m,
        &mut (*s).hasher_ as (*mut *mut u8),
        data as (*const u8),
        mask as (usize),
        &mut (*s).params as (*mut BrotliEncoderParams),
        wrapped_last_processed_pos as (usize),
        bytes as (usize),
        is_last
    );
    if !(0i32 == 0) {
        return 0i32;
    }
    if (*s).params.quality == 10i32 {
        0i32;
        BrotliCreateZopfliBackwardReferences(
            m,
            dictionary,
            bytes as (usize),
            wrapped_last_processed_pos as (usize),
            data as (*const u8),
            mask as (usize),
            &mut (*s).params as (*mut BrotliEncoderParams) as (*const BrotliEncoderParams),
            (*s).hasher_,
            (*s).dist_cache_.as_mut_ptr(),
            &mut (*s).last_insert_len_ as (*mut usize),
            &mut *(*s).commands_.offset(
                      (*s).num_commands_ as (isize)
                  ) as (*mut Command),
            &mut (*s).num_commands_ as (*mut usize),
            &mut (*s).num_literals_ as (*mut usize)
        );
        if !(0i32 == 0) {
            return 0i32;
        }
    } else if (*s).params.quality == 11i32 {
        0i32;
        BrotliCreateHqZopfliBackwardReferences(
            m,
            dictionary,
            bytes as (usize),
            wrapped_last_processed_pos as (usize),
            data as (*const u8),
            mask as (usize),
            &mut (*s).params as (*mut BrotliEncoderParams) as (*const BrotliEncoderParams),
            (*s).hasher_,
            (*s).dist_cache_.as_mut_ptr(),
            &mut (*s).last_insert_len_ as (*mut usize),
            &mut *(*s).commands_.offset(
                      (*s).num_commands_ as (isize)
                  ) as (*mut Command),
            &mut (*s).num_commands_ as (*mut usize),
            &mut (*s).num_literals_ as (*mut usize)
        );
        if !(0i32 == 0) {
            return 0i32;
        }
    } else {
        BrotliCreateBackwardReferences(
            dictionary,
            bytes as (usize),
            wrapped_last_processed_pos as (usize),
            data as (*const u8),
            mask as (usize),
            &mut (*s).params as (*mut BrotliEncoderParams) as (*const BrotliEncoderParams),
            (*s).hasher_,
            (*s).dist_cache_.as_mut_ptr(),
            &mut (*s).last_insert_len_ as (*mut usize),
            &mut *(*s).commands_.offset(
                      (*s).num_commands_ as (isize)
                  ) as (*mut Command),
            &mut (*s).num_commands_ as (*mut usize),
            &mut (*s).num_literals_ as (*mut usize)
        );
    }
    {
        let max_length
            : usize
            = MaxMetablockSize(
                  &mut (*s).params as (*mut BrotliEncoderParams) as (*const BrotliEncoderParams)
              );
        let max_literals
            : usize
            = max_length.wrapping_div(8i32 as (usize));
        let max_commands
            : usize
            = max_length.wrapping_div(8i32 as (usize));
        let processed_bytes
            : usize
            = (*s).input_pos_.wrapping_sub((*s).last_flush_pos_);
        let next_input_fits_metablock
            : i32
            = if !!(processed_bytes.wrapping_add(
                        InputBlockSize(s)
                    ) <= max_length) {
                  1i32
              } else {
                  0i32
              };
        let should_flush
            : i32
            = if !!((*s).params.quality < 4i32 && ((*s).num_literals_.wrapping_add(
                                                       (*s).num_commands_
                                                   ) >= 0x2fffi32 as (usize))) {
                  1i32
              } else {
                  0i32
              };
        if is_last == 0 && (force_flush == 0) && (should_flush == 0) && (next_input_fits_metablock != 0) && ((*s).num_literals_ < max_literals) && ((*s).num_commands_ < max_commands) {
            if UpdateLastProcessedPos(s) != 0 {
                HasherReset((*s).hasher_);
            }
            *out_size = 0i32 as (usize);
            return 1i32;
        }
    }
    if (*s).last_insert_len_ > 0i32 as (usize) {
        InitInsertCommand(
            &mut *(*s).commands_.offset(
                      {
                          let _old = (*s).num_commands_;
                          (*s).num_commands_ = (*s).num_commands_.wrapping_add(1 as (usize));
                          _old
                      } as (isize)
                  ) as (*mut Command),
            (*s).last_insert_len_
        );
        (*s).num_literals_ = (*s).num_literals_.wrapping_add(
                                 (*s).last_insert_len_
                             );
        (*s).last_insert_len_ = 0i32 as (usize);
    }
    if is_last == 0 && ((*s).input_pos_ == (*s).last_flush_pos_) {
        *out_size = 0i32 as (usize);
        return 1i32;
    }
    0i32;
    0i32;
    0i32;
    {
        let metablock_size
            : u32
            = (*s).input_pos_.wrapping_sub((*s).last_flush_pos_) as (u32);
        let mut storage
            : *mut u8
            = GetBrotliStorage(
                  s,
                  (2i32 as (u32)).wrapping_mul(metablock_size).wrapping_add(
                      502i32 as (u32)
                  ) as (usize)
              );
        let mut storage_ix : usize = (*s).last_byte_bits_ as (usize);
        if !(0i32 == 0) {
            return 0i32;
        }
        *storage.offset(0i32 as (isize)) = (*s).last_byte_;
        WriteMetaBlockInternal(
            m,
            data as (*const u8),
            mask as (usize),
            (*s).last_flush_pos_,
            metablock_size as (usize),
            is_last,
            &mut (*s).params as (*mut BrotliEncoderParams) as (*const BrotliEncoderParams),
            (*s).prev_byte_,
            (*s).prev_byte2_,
            (*s).num_literals_,
            (*s).num_commands_,
            (*s).commands_,
            (*s).saved_dist_cache_.as_mut_ptr() as (*const i32),
            (*s).dist_cache_.as_mut_ptr(),
            &mut storage_ix as (*mut usize),
            storage
        );
        if !(0i32 == 0) {
            return 0i32;
        }
        (*s).last_byte_ = *storage.offset((storage_ix >> 3i32) as (isize));
        (*s).last_byte_bits_ = (storage_ix & 7u32 as (usize)) as (u8);
        (*s).last_flush_pos_ = (*s).input_pos_;
        if UpdateLastProcessedPos(s) != 0 {
            HasherReset((*s).hasher_);
        }
        if (*s).last_flush_pos_ > 0i32 as (usize) {
            (*s).prev_byte_ = *data.offset(
                                   (((*s).last_flush_pos_ as (u32)).wrapping_sub(
                                        1i32 as (u32)
                                    ) & mask) as (isize)
                               );
        }
        if (*s).last_flush_pos_ > 1i32 as (usize) {
            (*s).prev_byte2_ = *data.offset(
                                    ((*s).last_flush_pos_.wrapping_sub(
                                         2i32 as (usize)
                                     ) as (u32) & mask) as (isize)
                                );
        }
        (*s).num_commands_ = 0i32 as (usize);
        (*s).num_literals_ = 0i32 as (usize);
        memcpy(
            (*s).saved_dist_cache_.as_mut_ptr(
            ) as (*mut ::std::os::raw::c_void),
            (*s).dist_cache_.as_mut_ptr() as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<[i32; 4]>()
        );
        *output = &mut *storage.offset(0i32 as (isize)) as (*mut u8);
        *out_size = storage_ix >> 3i32;
        1i32
    }
}

unsafe extern fn WriteMetadataHeader(
    mut s : *mut BrotliEncoderStateStruct,
    block_size : usize,
    mut header : *mut u8
) -> usize {
    let mut storage_ix : usize;
    storage_ix = (*s).last_byte_bits_ as (usize);
    *header.offset(0i32 as (isize)) = (*s).last_byte_;
    (*s).last_byte_ = 0i32 as (u8);
    (*s).last_byte_bits_ = 0i32 as (u8);
    BrotliWriteBits(
        1i32 as (usize),
        0i32 as (usize),
        &mut storage_ix as (*mut usize),
        header
    );
    BrotliWriteBits(
        2i32 as (usize),
        3i32 as (usize),
        &mut storage_ix as (*mut usize),
        header
    );
    BrotliWriteBits(
        1i32 as (usize),
        0i32 as (usize),
        &mut storage_ix as (*mut usize),
        header
    );
    if block_size == 0i32 as (usize) {
        BrotliWriteBits(
            2i32 as (usize),
            0i32 as (usize),
            &mut storage_ix as (*mut usize),
            header
        );
    } else {
        let mut nbits
            : u32
            = if block_size == 1i32 as (usize) {
                  0i32 as (u32)
              } else {
                  Log2FloorNonZero(
                      (block_size as (u32)).wrapping_sub(1i32 as (u32)) as (usize)
                  ).wrapping_add(
                      1i32 as (u32)
                  )
              };
        let mut nbytes
            : u32
            = nbits.wrapping_add(7i32 as (u32)).wrapping_div(8i32 as (u32));
        BrotliWriteBits(
            2i32 as (usize),
            nbytes as (usize),
            &mut storage_ix as (*mut usize),
            header
        );
        BrotliWriteBits(
            (8i32 as (u32)).wrapping_mul(nbytes) as (usize),
            block_size.wrapping_sub(1i32 as (usize)),
            &mut storage_ix as (*mut usize),
            header
        );
    }
    storage_ix.wrapping_add(7u32 as (usize)) >> 3i32
}

unsafe extern fn brotli_min_uint32_t(
    mut a : u32, mut b : u32
) -> u32 {
    if a < b { a } else { b }
}

unsafe extern fn ProcessMetadata(
    mut s : *mut BrotliEncoderStateStruct,
    mut available_in : *mut usize,
    mut next_in : *mut *const u8,
    mut available_out : *mut usize,
    mut next_out : *mut *mut u8,
    mut total_out : *mut usize
) -> i32 {
    if *available_in > (1u32 << 24i32) as (usize) {
        return 0i32;
    }
    if (*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING as (i32) {
        (*s).remaining_metadata_bytes_ = *available_in as (u32);
        (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_METADATA_HEAD;
    }
    if (*s).stream_state_ as (i32) != BrotliEncoderStreamState::BROTLI_STREAM_METADATA_HEAD as (i32) && ((*s).stream_state_ as (i32) != BrotliEncoderStreamState::BROTLI_STREAM_METADATA_BODY as (i32)) {
        return 0i32;
    }
    while 1i32 != 0 {
        if InjectFlushOrPushOutput(
               s,
               available_out,
               next_out,
               total_out
           ) != 0 {
            if 1337i32 != 0 {
                continue;
            }
        }
        if (*s).available_out_ != 0i32 as (usize) {
            if 1337i32 != 0 {
                break;
            }
        }
        if (*s).input_pos_ != (*s).last_flush_pos_ {
            let mut result
                : i32
                = EncodeData(
                      s,
                      0i32,
                      1i32,
                      &mut (*s).available_out_ as (*mut usize),
                      &mut (*s).next_out_ as (*mut *mut u8)
                  );
            if result == 0 {
                return 0i32;
            }
            {
                if 1337i32 != 0 {
                    continue;
                }
            }
        }
        if (*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_METADATA_HEAD as (i32) {
            (*s).next_out_ = (*s).tiny_buf_.u8.as_mut_ptr();
            (*s).available_out_ = WriteMetadataHeader(
                                      s,
                                      (*s).remaining_metadata_bytes_ as (usize),
                                      (*s).next_out_
                                  );
            (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_METADATA_BODY;
            {
                if 1337i32 != 0 {
                    continue;
                }
            }
        } else {
            if (*s).remaining_metadata_bytes_ == 0i32 as (u32) {
                (*s).remaining_metadata_bytes_ = !(0i32 as (u32));
                (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING;
                {
                    if 1337i32 != 0 {
                        break;
                    }
                }
            }
            if *available_out != 0 {
                let mut copy
                    : u32
                    = brotli_min_size_t(
                          (*s).remaining_metadata_bytes_ as (usize),
                          *available_out
                      ) as (u32);
                memcpy(
                    *next_out as (*mut ::std::os::raw::c_void),
                    *next_in as (*const ::std::os::raw::c_void),
                    copy as (usize)
                );
                *next_in = (*next_in).offset(copy as (isize));
                *available_in = (*available_in).wrapping_sub(copy as (usize));
                (*s).remaining_metadata_bytes_ = (*s).remaining_metadata_bytes_.wrapping_sub(
                                                     copy
                                                 );
                *next_out = (*next_out).offset(copy as (isize));
                *available_out = (*available_out).wrapping_sub(copy as (usize));
            } else {
                let mut copy
                    : u32
                    = brotli_min_uint32_t(
                          (*s).remaining_metadata_bytes_,
                          16i32 as (u32)
                      );
                (*s).next_out_ = (*s).tiny_buf_.u8.as_mut_ptr();
                memcpy(
                    (*s).next_out_ as (*mut ::std::os::raw::c_void),
                    *next_in as (*const ::std::os::raw::c_void),
                    copy as (usize)
                );
                *next_in = (*next_in).offset(copy as (isize));
                *available_in = (*available_in).wrapping_sub(copy as (usize));
                (*s).remaining_metadata_bytes_ = (*s).remaining_metadata_bytes_.wrapping_sub(
                                                     copy
                                                 );
                (*s).available_out_ = copy as (usize);
            }
            {
                if 1337i32 != 0 {
                    continue;
                }
            }
        }
    }
    1i32
}

unsafe extern fn CheckFlushComplete(
    mut s : *mut BrotliEncoderStateStruct
) { if (*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED as (i32) && ((*s).available_out_ == 0i32 as (usize)) {
        (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING;
        (*s).next_out_ = 0i32 as (*mut u8);
    }
}

unsafe extern fn BrotliEncoderCompressStreamFast(
    mut s : *mut BrotliEncoderStateStruct,
    mut op : BrotliEncoderOperation,
    mut available_in : *mut usize,
    mut next_in : *mut *const u8,
    mut available_out : *mut usize,
    mut next_out : *mut *mut u8,
    mut total_out : *mut usize
) -> i32 {
    let block_size_limit
        : usize
        = 1i32 as (usize) << (*s).params.lgwin;
    let buf_size
        : usize
        = brotli_min_size_t(
              kCompressFragmentTwoPassBlockSize,
              brotli_min_size_t(*available_in,block_size_limit)
          );
    let mut tmp_command_buf
        : *mut u32
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
    let mut command_buf
        : *mut u32
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
    let mut tmp_literal_buf
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut literal_buf
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut m
        : *mut MemoryManager
        = &mut (*s).memory_manager_ as (*mut MemoryManager);
    if (*s).params.quality != 0i32 && ((*s).params.quality != 1i32) {
        return 0i32;
    }
    if (*s).params.quality == 1i32 {
        if (*s).command_buf_.is_null(
           ) && (buf_size == kCompressFragmentTwoPassBlockSize) {
            (*s).command_buf_ = if kCompressFragmentTwoPassBlockSize != 0 {
                                    BrotliAllocate(
                                        m,
                                        kCompressFragmentTwoPassBlockSize.wrapping_mul(
                                            ::std::mem::size_of::<u32>()
                                        )
                                    ) as (*mut u32)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                                };
            (*s).literal_buf_ = if kCompressFragmentTwoPassBlockSize != 0 {
                                    BrotliAllocate(
                                        m,
                                        kCompressFragmentTwoPassBlockSize.wrapping_mul(
                                            ::std::mem::size_of::<u8>()
                                        )
                                    ) as (*mut u8)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                                };
            if !(0i32 == 0) {
                return 0i32;
            }
        }
        if !(*s).command_buf_.is_null() {
            command_buf = (*s).command_buf_;
            literal_buf = (*s).literal_buf_;
        } else {
            tmp_command_buf = if buf_size != 0 {
                                  BrotliAllocate(
                                      m,
                                      buf_size.wrapping_mul(::std::mem::size_of::<u32>())
                                  ) as (*mut u32)
                              } else {
                                  0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                              };
            tmp_literal_buf = if buf_size != 0 {
                                  BrotliAllocate(
                                      m,
                                      buf_size.wrapping_mul(::std::mem::size_of::<u8>())
                                  ) as (*mut u8)
                              } else {
                                  0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                              };
            if !(0i32 == 0) {
                return 0i32;
            }
            command_buf = tmp_command_buf;
            literal_buf = tmp_literal_buf;
        }
    }
    while 1i32 != 0 {
        if InjectFlushOrPushOutput(
               s,
               available_out,
               next_out,
               total_out
           ) != 0 {
            if 1337i32 != 0 {
                continue;
            }
        }
        if (*s).available_out_ == 0i32 as (usize) && ((*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING as (i32)) && (*available_in != 0i32 as (usize) || op as (i32) != BrotliEncoderOperation::BROTLI_OPERATION_PROCESS as (i32)) {
            let mut block_size
                : usize
                = brotli_min_size_t(block_size_limit,*available_in);
            let mut is_last
                : i32
                = (*available_in == block_size && (op as (i32) == BrotliEncoderOperation::BROTLI_OPERATION_FINISH as (i32))) as (i32);
            let mut force_flush
                : i32
                = (*available_in == block_size && (op as (i32) == BrotliEncoderOperation::BROTLI_OPERATION_FLUSH as (i32))) as (i32);
            let mut max_out_size
                : usize
                = (2i32 as (usize)).wrapping_mul(block_size).wrapping_add(
                      502i32 as (usize)
                  );
            let mut inplace : i32 = 1i32;
            let mut storage
                : *mut u8
                = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
            let mut storage_ix : usize = (*s).last_byte_bits_ as (usize);
            let mut table_size : usize;
            let mut table : *mut i32;
            if force_flush != 0 && (block_size == 0i32 as (usize)) {
                (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED;
                {
                    if 1337i32 != 0 {
                        continue;
                    }
                }
            }
            if max_out_size <= *available_out {
                storage = *next_out;
            } else {
                inplace = 0i32;
                storage = GetBrotliStorage(s,max_out_size);
                if !(0i32 == 0) {
                    return 0i32;
                }
            }
            *storage.offset(0i32 as (isize)) = (*s).last_byte_;
            table = GetHashTable(
                        s,
                        (*s).params.quality,
                        block_size,
                        &mut table_size as (*mut usize)
                    );
            if !(0i32 == 0) {
                return 0i32;
            }
            if (*s).params.quality == 0i32 {
                BrotliCompressFragmentFast(
                    m,
                    *next_in,
                    block_size,
                    is_last,
                    table,
                    table_size,
                    (*s).cmd_depths_.as_mut_ptr(),
                    (*s).cmd_bits_.as_mut_ptr(),
                    &mut (*s).cmd_code_numbits_ as (*mut usize),
                    (*s).cmd_code_.as_mut_ptr(),
                    &mut storage_ix as (*mut usize),
                    storage
                );
                if !(0i32 == 0) {
                    return 0i32;
                }
            } else {
                BrotliCompressFragmentTwoPass(
                    m,
                    *next_in,
                    block_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    table_size,
                    &mut storage_ix as (*mut usize),
                    storage
                );
                if !(0i32 == 0) {
                    return 0i32;
                }
            }
            *next_in = (*next_in).offset(block_size as (isize));
            *available_in = (*available_in).wrapping_sub(block_size);
            if inplace != 0 {
                let mut out_bytes : usize = storage_ix >> 3i32;
                0i32;
                0i32;
                *next_out = (*next_out).offset(out_bytes as (isize));
                *available_out = (*available_out).wrapping_sub(out_bytes);
                (*s).total_out_ = (*s).total_out_.wrapping_add(out_bytes);
                if !total_out.is_null() {
                    *total_out = (*s).total_out_;
                }
            } else {
                let mut out_bytes : usize = storage_ix >> 3i32;
                (*s).next_out_ = storage;
                (*s).available_out_ = out_bytes;
            }
            (*s).last_byte_ = *storage.offset((storage_ix >> 3i32) as (isize));
            (*s).last_byte_bits_ = (storage_ix & 7u32 as (usize)) as (u8);
            if force_flush != 0 {
                (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED;
            }
            if is_last != 0 {
                (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_FINISHED;
            }
            {
                if 1337i32 != 0 {
                    continue;
                }
            }
        }
        {
            if 1337i32 != 0 {
                break;
            }
        }
    }
    {
        BrotliFree(m,tmp_command_buf as (*mut ::std::os::raw::c_void));
        tmp_command_buf = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
    }
    {
        BrotliFree(m,tmp_literal_buf as (*mut ::std::os::raw::c_void));
        tmp_literal_buf = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    CheckFlushComplete(s);
    1i32
}

unsafe extern fn RemainingInputBlockSize(
    mut s : *mut BrotliEncoderStateStruct
) -> usize {
    let delta : usize = UnprocessedInputSize(s);
    let mut block_size : usize = InputBlockSize(s);
    if delta >= block_size {
        return 0i32 as (usize);
    }
    block_size.wrapping_sub(delta)
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderCompressStream(
    mut s : *mut BrotliEncoderStateStruct,
    mut op : BrotliEncoderOperation,
    mut available_in : *mut usize,
    mut next_in : *mut *const u8,
    mut available_out : *mut usize,
    mut next_out : *mut *mut u8,
    mut total_out : *mut usize
) -> i32 {
    if EnsureInitialized(s) == 0 {
        return 0i32;
    }
    if (*s).remaining_metadata_bytes_ != !(0i32 as (u32)) {
        if *available_in != (*s).remaining_metadata_bytes_ as (usize) {
            return 0i32;
        }
        if op as (i32) != BrotliEncoderOperation::BROTLI_OPERATION_EMIT_METADATA as (i32) {
            return 0i32;
        }
    }
    if op as (i32) == BrotliEncoderOperation::BROTLI_OPERATION_EMIT_METADATA as (i32) {
        UpdateSizeHint(s,0i32 as (usize));
        return
            ProcessMetadata(
                s,
                available_in,
                next_in,
                available_out,
                next_out,
                total_out
            );
    }
    if (*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_METADATA_HEAD as (i32) || (*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_METADATA_BODY as (i32) {
        return 0i32;
    }
    if (*s).stream_state_ as (i32) != BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING as (i32) && (*available_in != 0i32 as (usize)) {
        return 0i32;
    }
    if (*s).params.quality == 0i32 || (*s).params.quality == 1i32 {
        return
            BrotliEncoderCompressStreamFast(
                s,
                op,
                available_in,
                next_in,
                available_out,
                next_out,
                total_out
            );
    }
    while 1i32 != 0 {
        let mut remaining_block_size : usize = RemainingInputBlockSize(s);
        if remaining_block_size != 0i32 as (usize) && (*available_in != 0i32 as (usize)) {
            let mut copy_input_size
                : usize
                = brotli_min_size_t(remaining_block_size,*available_in);
            CopyInputToRingBuffer(s,copy_input_size,*next_in);
            *next_in = (*next_in).offset(copy_input_size as (isize));
            *available_in = (*available_in).wrapping_sub(copy_input_size);
            {
                if 1337i32 != 0 {
                    continue;
                }
            }
        }
        if InjectFlushOrPushOutput(
               s,
               available_out,
               next_out,
               total_out
           ) != 0 {
            if 1337i32 != 0 {
                continue;
            }
        }
        if (*s).available_out_ == 0i32 as (usize) && ((*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING as (i32)) {
            if remaining_block_size == 0i32 as (usize) || op as (i32) != BrotliEncoderOperation::BROTLI_OPERATION_PROCESS as (i32) {
                let mut is_last
                    : i32
                    = if !!(*available_in == 0i32 as (usize) && (op as (i32) == BrotliEncoderOperation::BROTLI_OPERATION_FINISH as (i32))) {
                          1i32
                      } else {
                          0i32
                      };
                let mut force_flush
                    : i32
                    = if !!(*available_in == 0i32 as (usize) && (op as (i32) == BrotliEncoderOperation::BROTLI_OPERATION_FLUSH as (i32))) {
                          1i32
                      } else {
                          0i32
                      };
                let mut result : i32;
                UpdateSizeHint(s,*available_in);
                result = EncodeData(
                             s,
                             is_last,
                             force_flush,
                             &mut (*s).available_out_ as (*mut usize),
                             &mut (*s).next_out_ as (*mut *mut u8)
                         );
                if result == 0 {
                    return 0i32;
                }
                if force_flush != 0 {
                    (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED;
                }
                if is_last != 0 {
                    (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_FINISHED;
                }
                {
                    if 1337i32 != 0 {
                        continue;
                    }
                }
            }
        }
        {
            if 1337i32 != 0 {
                break;
            }
        }
    }
    CheckFlushComplete(s);
    1i32
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderIsFinished(
    mut s : *mut BrotliEncoderStateStruct
) -> i32 {
    if !!((*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_FINISHED as (i32) && (BrotliEncoderHasMoreOutput(
                                                                                                           s
                                                                                                       ) == 0)) {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderHasMoreOutput(
    mut s : *mut BrotliEncoderStateStruct
) -> i32 {
    if !!((*s).available_out_ != 0i32 as (usize)) {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderTakeOutput(
    mut s : *mut BrotliEncoderStateStruct, mut size : *mut usize
) -> *const u8 {
    let mut consumed_size : usize = (*s).available_out_;
    let mut result : *mut u8 = (*s).next_out_;
    if *size != 0 {
        consumed_size = brotli_min_size_t(*size,(*s).available_out_);
    }
    if consumed_size != 0 {
        (*s).next_out_ = (*s).next_out_.offset(consumed_size as (isize));
        (*s).available_out_ = (*s).available_out_.wrapping_sub(
                                  consumed_size
                              );
        (*s).total_out_ = (*s).total_out_.wrapping_add(consumed_size);
        CheckFlushComplete(s);
        *size = consumed_size;
    } else {
        *size = 0i32 as (usize);
        result = 0i32 as (*mut u8);
    }
    result as (*const u8)
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderVersion() -> u32 {
    0x1000000i32 as (u32)
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderInputBlockSize(
    mut s : *mut BrotliEncoderStateStruct
) -> usize {
    InputBlockSize(s)
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderCopyInputToRingBuffer(
    mut s : *mut BrotliEncoderStateStruct,
    input_size : usize,
    mut input_buffer : *const u8
) {
    CopyInputToRingBuffer(s,input_size,input_buffer);
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderWriteData(
    mut s : *mut BrotliEncoderStateStruct,
    is_last : i32,
    force_flush : i32,
    mut out_size : *mut usize,
    mut output : *mut *mut u8
) -> i32 {
    EncodeData(s,is_last,force_flush,out_size,output)
}
