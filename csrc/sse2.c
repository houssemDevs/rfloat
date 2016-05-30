
typedef struct {
	double v0;
	double v1;
} f64x2 __attribute__((aligned(16)));


f64x2 _add_f64x2(f64x2 a, f64x2 b) {
	__asm__ (
		"movapd %0, %%xmm0;"
		"addpd %1, %%xmm0;"
		"movapd %%xmm0, %0;"
		:"=m"(b)
		:"m"(a),"m"(b)
		:"xmm0"
	);
	return b;
}

f64x2 _sub_f64x2(f64x2 a, f64x2 b) {
	__asm__ (
		"movapd %1, %%xmm0;"
		"subpd %0, %%xmm0;"
		"movapd %%xmm0, %0;"
		:"=m"(b)
		:"m"(a),"m"(b)
		:"xmm0"
	);
	return b;
}

f64x2 _mul_f64x2(f64x2 a, f64x2 b) {
	__asm__ (
		"movapd %0, %%xmm0;"
		"mulpd %1, %%xmm0;"
		"movapd %%xmm0, %0;"
		:"=m"(b)
		:"m"(a),"m"(b)
		:"xmm0"
	);
	return b;
}

f64x2 _div_f64x2(f64x2 a, f64x2 b) {
	__asm__ (
		"movapd %1, %%xmm0;"
		"divpd %0, %%xmm0;"
		"movapd %%xmm0, %0;"
		:"=m"(b)
		:"m"(a),"m"(b)
		:"xmm0"
	);
	return b;
}

f64x2 _sqrt_f64x2(f64x2 a) {
	__asm__ (
		"sqrtpd %0, %%xmm0;"
		"movapd %%xmm0, %0;"
		:"=m"(a)
		:"m"(a)
		:"xmm0"
	);
	return a;
}

f64x2 _max_f64x2(f64x2 a, f64x2 b) {
	__asm__ (
		"movapd %0, %%xmm0;"
		"maxpd %1, %%xmm0;"
		"movapd %%xmm0, %0;"
		:"=m"(b)
		:"m"(a),"m"(b)
		:"xmm0"
	);
	return b;
}

f64x2 _min_f64x2(f64x2 a, f64x2 b) {
	__asm__ (
		"movapd %0, %%xmm0;"
		"minpd %1, %%xmm0;"
		"movapd %%xmm0, %0;"
		:"=m"(b)
		:"m"(a),"m"(b)
		:"xmm0"
	);
	return b;
}

