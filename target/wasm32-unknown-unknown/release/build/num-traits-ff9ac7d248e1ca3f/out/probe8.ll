; ModuleID = 'probe8.518ed467-cgu.0'
source_filename = "probe8.518ed467-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]" = type {}

; core::intrinsics::const_eval_select
; Function Attrs: inlinehint nounwind
define hidden i64 @_ZN4core10intrinsics17const_eval_select17hf9251ebe7154ef24E(double %arg) unnamed_addr #0 {
start:
; call core::ops::function::FnOnce::call_once
  %0 = call i64 @_ZN4core3ops8function6FnOnce9call_once17h285e11574e6c7647E(double %arg) #2
  br label %bb1

bb1:                                              ; preds = %start
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i64 %0
}

; core::f64::<impl f64>::to_ne_bytes
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h24afe84d639b88c8E"([8 x i8]* sret([8 x i8]) %0, double %self) unnamed_addr #0 {
start:
; call core::f64::<impl f64>::to_bits
  %_2 = call i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits17h217a0e254eccc937E"(double %self) #2
  br label %bb1

bb1:                                              ; preds = %start
; call core::num::<impl u64>::to_ne_bytes
  call void @"_ZN4core3num21_$LT$impl$u20$u64$GT$11to_ne_bytes17hf083c0ffc5915152E"([8 x i8]* sret([8 x i8]) %0, i64 %_2) #2
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; core::f64::<impl f64>::to_bits
; Function Attrs: inlinehint nounwind
define internal i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits17h217a0e254eccc937E"(double %self) unnamed_addr #0 {
start:
  %_3 = alloca double, align 8
  store double %self, double* %_3, align 8
  %0 = load double, double* %_3, align 8
; call core::intrinsics::const_eval_select
  %1 = call i64 @_ZN4core10intrinsics17const_eval_select17hf9251ebe7154ef24E(double %0) #2
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::f64::<impl f64>::to_bits::{{closure}}
; Function Attrs: inlinehint nounwind
define hidden i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits28_$u7b$$u7b$closure$u7d$$u7d$17h81d8fb57b3856837E"(%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]"* align 1 %_1, double %rt) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = bitcast double %rt to i64
  store i64 %1, i64* %0, align 8
  %2 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %2
}

; core::num::<impl u64>::to_ne_bytes
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core3num21_$LT$impl$u20$u64$GT$11to_ne_bytes17hf083c0ffc5915152E"([8 x i8]* sret([8 x i8]) %0, i64 %self) unnamed_addr #0 {
start:
  %1 = bitcast [8 x i8]* %0 to i64*
  store i64 %self, i64* %1, align 1
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind
define internal i64 @_ZN4core3ops8function6FnOnce9call_once17h285e11574e6c7647E(double %0) unnamed_addr #0 {
start:
  %_2 = alloca double, align 8
  %_1 = alloca %"[closure@core::f64::<impl f64>::to_bits::{closure#0}]", align 1
  store double %0, double* %_2, align 8
  %1 = load double, double* %_2, align 8
; call core::f64::<impl f64>::to_bits::{{closure}}
  %2 = call i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits28_$u7b$$u7b$closure$u7d$$u7d$17h81d8fb57b3856837E"(%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]"* align 1 %_1, double %1) #2
  br label %bb1

bb1:                                              ; preds = %start
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i64 %2
}

; probe8::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe85probe17h6a618935d3556893E() unnamed_addr #1 {
start:
  %_1 = alloca [8 x i8], align 1
; call core::f64::<impl f64>::to_ne_bytes
  call void @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h24afe84d639b88c8E"([8 x i8]* sret([8 x i8]) %_1, double 3.140000e+00) #2
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

attributes #0 = { inlinehint nounwind "target-cpu"="generic" }
attributes #1 = { nounwind "target-cpu"="generic" }
attributes #2 = { nounwind }
