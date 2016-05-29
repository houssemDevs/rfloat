
#include <stdio.h>

typedef struct {
	float v0;
	float v1;
} f32x2 __attribute__((aligned(16)));

f32x2 _add_f32x2(f32x2 a , f32x2 b ) {
	__asm__ (
		"movaps %0, %%xmm0;"
		"addps %1, %%xmm0;"
		"movaps %%xmm0, %0;"
		:"=m"(b)
		:"m"(a), "m"(b)
		:"xmm0"
		);
	return b;
}

f32x2 _sub_f32x2(f32x2 a , f32x2 b ) {
	__asm__ (
		"movaps %1, %%xmm0;"
		"subps %0, %%xmm0;"
		"movaps %%xmm0, %0;"
		:"=m"(b)
		:"m"(a), "m"(b)
		:"xmm0"
		);
	return b;
}

f32x2 _mul_f32x2(f32x2 a , f32x2 b ) {
	__asm__ (
		"movaps %0, %%xmm0;"
		"mulps %1, %%xmm0;"
		"movaps %%xmm0, %0;"
		:"=m"(b)
		:"m"(a), "m"(b)
		:"xmm0"
		);
	return b;
}

f32x2 _div_f32x2(f32x2 a , f32x2 b ) {
	__asm__ (
		"movaps %1, %%xmm0;"
		"divps %0, %%xmm0;"
		"movaps %%xmm0, %0;"
		:"=m"(b)
		:"m"(a), "m"(b)
		:"xmm0"
		);
	return b;
}

f32x2 _max_f32x2(f32x2 a , f32x2 b ) {
	__asm__ (
		"movaps %0, %%xmm0;"
		"maxps %1, %%xmm0;"
		"movaps %%xmm0, %0;"
		:"=m"(b)
		:"m"(a), "m"(b)
		:"xmm0"
		);
	return b;
}

f32x2 _min_f32x2(f32x2 a , f32x2 b ) {
	__asm__ (
		"movaps %0, %%xmm0;"
		"minps %1, %%xmm0;"
		"movaps %%xmm0, %0;"
		:"=m"(b)
		:"m"(a), "m"(b)
		:"xmm0"
		);
	return b;
}

f32x2 _sqrt_f32x2(f32x2 a) {
	__asm__ (
		"sqrtps %0, %%xmm0;"
		"movaps %%xmm0, %0;"
		:"=m"(a)
		:"m"(a)
		:"xmm0"
		);
	return a;
}
