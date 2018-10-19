struct VecU8 {
    unsigned char *data;
    size_t size;
};
struct VecU8 new_vec_u8() {
    struct VecU8 ret;
    ret.data = NULL;
    ret.size = 0;
    return ret;
}
uint64_t round_up_to_power_of_two(uint64_t v) {
    v--;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    {
        uint64_t tmp = v;
        tmp >>= 32;
        v |= tmp;
    }
    v++;
    return v;
}

void push_vec_u8(struct VecU8 *thus, const unsigned char*data, size_t size) {
    size_t new_actual_size = thus->size + size;
    if (size == 0 || new_actual_size < thus->size) {
        return;
    }
    {
        size_t new_alloc_size = round_up_to_power_of_two(new_actual_size);
        size_t old_alloc_size = round_up_to_power_of_two(thus->size);
        if (thus->size == 0 || old_alloc_size != new_alloc_size ) {
            unsigned char *tmp = custom_malloc_f(custom_alloc_opaque, new_alloc_size);
            size_t to_copy = old_alloc_size;
            if (new_alloc_size < old_alloc_size) {
                to_copy = new_alloc_size;
            }
            memcpy(tmp, thus->data, to_copy);
            custom_free_f(custom_alloc_opaque, thus->data);
            thus->data = tmp;
        }
        if (new_alloc_size < new_actual_size) {
            abort(); // assert
        }
        memcpy(thus->data + thus->size, data, size);
        thus->size = new_actual_size;
    }
}

void release_vec_u8(struct VecU8 *thus) {
    if (thus->size) {
        custom_free_f(custom_alloc_opaque, thus->data);
        thus->size = 0;
        thus->data = NULL;
    }
}


void trunc_vec_u8(struct VecU8 *thus, size_t new_len) {
    if (thus->size > new_len) {
        thus->size = new_len;
    }
}
void reinit_vec_u8(struct VecU8 *thus, size_t new_len) {
    release_vec_u8(thus);
    thus->data = custom_malloc_f(custom_alloc_opaque, new_len);
    thus->size = new_len;
}
