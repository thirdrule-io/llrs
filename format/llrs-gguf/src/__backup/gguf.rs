// Group	        Types in Group	                    Description
// Float	        F32, F16, BF16, F64	                Standard floating-point
// Quantized (Q)	Q4_0, Q4_1, Q5_0, Q8_0, Q6_K, etc.	Quantization formats
// Integer	        I8, I16, I32, I64	                Basic signed ints
// IQ (int+quant)	IQ1_S, IQ2_K, IQ4_XS, etc.	        More efficient integer formats for quantized weights
// TQ	            TQ1_0, TQ2_0	                    Tensor quant formats for future/backed layouts

// Type	            Bits per weight	Notes
// Q4_K	            4-bit grouped quant	                Very common
// Q6_K	            6-bit grouped quant	                Better accuracy
// Q8_0	            8-bit quantized	                    Almost float-like
// IQ*	            Int-Quant hybrid	                Super compact, newer
// F32	            32-bit float	                    Ground truth format

#[derive(Debug)]
pub enum GgmlType {
    F32 = 0,
    F16 = 1,
    Q4_0 = 2,
    Q4_1 = 3,
    Q4_2 = 4,
    Q4_3 = 5,
    Q5_0 = 6,
    Q5_1 = 7,
    Q8_0 = 8,
    Q8_1 = 9,
    Q2_K = 10,
    Q3_K = 11,
    Q4_K = 12,
    Q5_K = 13,
    Q6_K = 14,
    Q8_K = 15,
    Iq2_XXS = 16,
    Iq2_XS = 17,
    IQ3_XXS = 18,
    Iq1_S = 19,
    Iq4_NL = 20,
    IQ3_S = 21,
    IQ2_S = 22,
    IQ4_XS = 23,
    I8 = 24,
    I16 = 25,
    I32 = 26,
    I64 = 27,
    F64 = 28,
    IQ1_M = 29,
    BF16 = 30,
    TQ1_0 = 34,
    TQ2_0 = 35,
    COUNT = 39,
}
