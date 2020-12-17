//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
// AUTOMATICALLY GENERATED : Don't edit by hand.
// See usd-basic-types

use super::Value;
use cpp::*;

use half::f16; // Half is not a standard rust type

// To avoid a conflict between types, like vec4 and quat, we use tuple structs.
// repr(transparent) ensures that the struct is exactly the same size in memory
// as the type it is wrapping. This allows us to safely cast forwards and
// backwards between the types.
//
// We provide a 'From<&T>' implementation for the tuple structs to
// provide a means to do a zero copy transfer from rust over to c++.
#[repr(transparent)]
pub struct Bool(pub bool);

impl From<&bool> for &Bool {
    fn from(other : &bool) -> Self {
        unsafe { &*((other as *const bool) as *const Bool) }
    }
}

cpp_class!(pub unsafe struct ArrayBool as "pxr::VtArray<bool>");

impl VtArray<bool> for ArrayBool {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayBool as "pxr::VtArray<bool>" {
                return pxr::VtArray<bool>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<bool> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<bool> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &bool) {
        unsafe {
            cpp!([self as "pxr::VtArray<bool> *",
                  elem as "const bool *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayBool {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayBool");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<bool> *",
                  index as "size_t"]
                -> * const bool as "const bool *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayBool {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayBool");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<bool> *",
                  index as "size_t"]
                -> * mut bool as "bool *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct UChar(pub u8);

impl From<&u8> for &UChar {
    fn from(other : &u8) -> Self {
        unsafe { &*((other as *const u8) as *const UChar) }
    }
}

cpp_class!(pub unsafe struct ArrayUChar as "pxr::VtArray<uint8_t>");

impl VtArray<u8> for ArrayUChar {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayUChar as "pxr::VtArray<uint8_t>" {
                return pxr::VtArray<uint8_t>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<uint8_t> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<uint8_t> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &u8) {
        unsafe {
            cpp!([self as "pxr::VtArray<uint8_t> *",
                  elem as "const uint8_t *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayUChar {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayUChar");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<uint8_t> *",
                  index as "size_t"]
                -> * const u8 as "const uint8_t *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayUChar {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayUChar");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<uint8_t> *",
                  index as "size_t"]
                -> * mut u8 as "uint8_t *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Int(pub i32);

impl From<&i32> for &Int {
    fn from(other : &i32) -> Self {
        unsafe { &*((other as *const i32) as *const Int) }
    }
}

cpp_class!(pub unsafe struct ArrayInt as "pxr::VtArray<int32_t>");

impl VtArray<i32> for ArrayInt {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayInt as "pxr::VtArray<int32_t>" {
                return pxr::VtArray<int32_t>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<int32_t> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<int32_t> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &i32) {
        unsafe {
            cpp!([self as "pxr::VtArray<int32_t> *",
                  elem as "const int32_t *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayInt {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayInt");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<int32_t> *",
                  index as "size_t"]
                -> * const i32 as "const int32_t *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayInt {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayInt");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<int32_t> *",
                  index as "size_t"]
                -> * mut i32 as "int32_t *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct UInt(pub u32);

impl From<&u32> for &UInt {
    fn from(other : &u32) -> Self {
        unsafe { &*((other as *const u32) as *const UInt) }
    }
}

cpp_class!(pub unsafe struct ArrayUInt as "pxr::VtArray<uint32_t>");

impl VtArray<u32> for ArrayUInt {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayUInt as "pxr::VtArray<uint32_t>" {
                return pxr::VtArray<uint32_t>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<uint32_t> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<uint32_t> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &u32) {
        unsafe {
            cpp!([self as "pxr::VtArray<uint32_t> *",
                  elem as "const uint32_t *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayUInt {
    type Output = u32;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayUInt");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<uint32_t> *",
                  index as "size_t"]
                -> * const u32 as "const uint32_t *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayUInt {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayUInt");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<uint32_t> *",
                  index as "size_t"]
                -> * mut u32 as "uint32_t *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Int64(pub i64);

impl From<&i64> for &Int64 {
    fn from(other : &i64) -> Self {
        unsafe { &*((other as *const i64) as *const Int64) }
    }
}

cpp_class!(pub unsafe struct ArrayInt64 as "pxr::VtArray<int64_t>");

impl VtArray<i64> for ArrayInt64 {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayInt64 as "pxr::VtArray<int64_t>" {
                return pxr::VtArray<int64_t>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<int64_t> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<int64_t> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &i64) {
        unsafe {
            cpp!([self as "pxr::VtArray<int64_t> *",
                  elem as "const int64_t *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayInt64 {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayInt64");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<int64_t> *",
                  index as "size_t"]
                -> * const i64 as "const int64_t *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayInt64 {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayInt64");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<int64_t> *",
                  index as "size_t"]
                -> * mut i64 as "int64_t *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct UInt64(pub u64);

impl From<&u64> for &UInt64 {
    fn from(other : &u64) -> Self {
        unsafe { &*((other as *const u64) as *const UInt64) }
    }
}

cpp_class!(pub unsafe struct ArrayUInt64 as "pxr::VtArray<uint64_t>");

impl VtArray<u64> for ArrayUInt64 {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayUInt64 as "pxr::VtArray<uint64_t>" {
                return pxr::VtArray<uint64_t>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<uint64_t> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<uint64_t> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &u64) {
        unsafe {
            cpp!([self as "pxr::VtArray<uint64_t> *",
                  elem as "const uint64_t *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayUInt64 {
    type Output = u64;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayUInt64");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<uint64_t> *",
                  index as "size_t"]
                -> * const u64 as "const uint64_t *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayUInt64 {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayUInt64");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<uint64_t> *",
                  index as "size_t"]
                -> * mut u64 as "uint64_t *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Half(pub f16);

impl From<&f16> for &Half {
    fn from(other : &f16) -> Self {
        unsafe { &*((other as *const f16) as *const Half) }
    }
}

cpp_class!(pub unsafe struct ArrayHalf as "pxr::VtArray<pxr::GfHalf>");

impl VtArray<f16> for ArrayHalf {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayHalf as "pxr::VtArray<pxr::GfHalf>" {
                return pxr::VtArray<pxr::GfHalf>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfHalf> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfHalf> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &f16) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfHalf> *",
                  elem as "const pxr::GfHalf *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayHalf {
    type Output = f16;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayHalf");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfHalf> *",
                  index as "size_t"]
                -> * const f16 as "const pxr::GfHalf *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayHalf {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayHalf");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfHalf> *",
                  index as "size_t"]
                -> * mut f16 as "pxr::GfHalf *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Float(pub f32);

impl From<&f32> for &Float {
    fn from(other : &f32) -> Self {
        unsafe { &*((other as *const f32) as *const Float) }
    }
}

cpp_class!(pub unsafe struct ArrayFloat as "pxr::VtArray<float>");

impl VtArray<f32> for ArrayFloat {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayFloat as "pxr::VtArray<float>" {
                return pxr::VtArray<float>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<float> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<float> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &f32) {
        unsafe {
            cpp!([self as "pxr::VtArray<float> *",
                  elem as "const float *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayFloat {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayFloat");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<float> *",
                  index as "size_t"]
                -> * const f32 as "const float *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayFloat {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayFloat");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<float> *",
                  index as "size_t"]
                -> * mut f32 as "float *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Double(pub f64);

impl From<&f64> for &Double {
    fn from(other : &f64) -> Self {
        unsafe { &*((other as *const f64) as *const Double) }
    }
}

cpp_class!(pub unsafe struct ArrayDouble as "pxr::VtArray<double>");

impl VtArray<f64> for ArrayDouble {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayDouble as "pxr::VtArray<double>" {
                return pxr::VtArray<double>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<double> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<double> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &f64) {
        unsafe {
            cpp!([self as "pxr::VtArray<double> *",
                  elem as "const double *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayDouble {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayDouble");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<double> *",
                  index as "size_t"]
                -> * const f64 as "const double *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayDouble {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayDouble");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<double> *",
                  index as "size_t"]
                -> * mut f64 as "double *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct TimeCode(pub crate::pxr::sdf::TimeCode);

impl From<&crate::pxr::sdf::TimeCode> for &TimeCode {
    fn from(other : &crate::pxr::sdf::TimeCode) -> Self {
        unsafe { &*((other as *const crate::pxr::sdf::TimeCode) as *const TimeCode) }
    }
}

cpp_class!(pub unsafe struct ArrayTimeCode as "pxr::VtArray<pxr::SdfTimeCode>");

impl VtArray<crate::pxr::sdf::TimeCode> for ArrayTimeCode {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayTimeCode as "pxr::VtArray<pxr::SdfTimeCode>" {
                return pxr::VtArray<pxr::SdfTimeCode>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::SdfTimeCode> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::SdfTimeCode> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &crate::pxr::sdf::TimeCode) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::SdfTimeCode> *",
                  elem as "const pxr::SdfTimeCode *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayTimeCode {
    type Output = crate::pxr::sdf::TimeCode;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayTimeCode");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::SdfTimeCode> *",
                  index as "size_t"]
                -> * const crate::pxr::sdf::TimeCode as "const pxr::SdfTimeCode *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayTimeCode {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayTimeCode");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::SdfTimeCode> *",
                  index as "size_t"]
                -> * mut crate::pxr::sdf::TimeCode as "pxr::SdfTimeCode *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Token(pub crate::pxr::tf::Token);

impl From<&crate::pxr::tf::Token> for &Token {
    fn from(other : &crate::pxr::tf::Token) -> Self {
        unsafe { &*((other as *const crate::pxr::tf::Token) as *const Token) }
    }
}

cpp_class!(pub unsafe struct ArrayToken as "pxr::VtArray<pxr::TfToken>");

impl VtArray<crate::pxr::tf::Token> for ArrayToken {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayToken as "pxr::VtArray<pxr::TfToken>" {
                return pxr::VtArray<pxr::TfToken>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::TfToken> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::TfToken> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &crate::pxr::tf::Token) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::TfToken> *",
                  elem as "const pxr::TfToken *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayToken {
    type Output = crate::pxr::tf::Token;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayToken");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::TfToken> *",
                  index as "size_t"]
                -> * const crate::pxr::tf::Token as "const pxr::TfToken *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayToken {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayToken");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::TfToken> *",
                  index as "size_t"]
                -> * mut crate::pxr::tf::Token as "pxr::TfToken *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Asset(pub crate::pxr::sdf::AsstPth);

impl From<&crate::pxr::sdf::AsstPth> for &Asset {
    fn from(other : &crate::pxr::sdf::AsstPth) -> Self {
        unsafe { &*((other as *const crate::pxr::sdf::AsstPth) as *const Asset) }
    }
}

cpp_class!(pub unsafe struct ArrayAsset as "pxr::VtArray<pxr::SdfAssetPath>");

impl VtArray<crate::pxr::sdf::AsstPth> for ArrayAsset {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayAsset as "pxr::VtArray<pxr::SdfAssetPath>" {
                return pxr::VtArray<pxr::SdfAssetPath>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::SdfAssetPath> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::SdfAssetPath> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &crate::pxr::sdf::AsstPth) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::SdfAssetPath> *",
                  elem as "const pxr::SdfAssetPath *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayAsset {
    type Output = crate::pxr::sdf::AsstPth;
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayAsset");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::SdfAssetPath> *",
                  index as "size_t"]
                -> * const crate::pxr::sdf::AsstPth as "const pxr::SdfAssetPath *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayAsset {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayAsset");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::SdfAssetPath> *",
                  index as "size_t"]
                -> * mut crate::pxr::sdf::AsstPth as "pxr::SdfAssetPath *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Matrix2d(pub [f64;2*3]);

impl From<&[f64;2*3]> for &Matrix2d {
    fn from(other : &[f64;2*3]) -> Self {
        unsafe { &*((other as *const [f64;2*3]) as *const Matrix2d) }
    }
}

cpp_class!(pub unsafe struct ArrayMatrix2d as "pxr::VtArray<pxr::GfMatrix2d>");

impl VtArray<[f64;2*3]> for ArrayMatrix2d {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayMatrix2d as "pxr::VtArray<pxr::GfMatrix2d>" {
                return pxr::VtArray<pxr::GfMatrix2d>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfMatrix2d> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfMatrix2d> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f64;2*3]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfMatrix2d> *",
                  elem as "const pxr::GfMatrix2d *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayMatrix2d {
    type Output = [f64;2*3];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayMatrix2d");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfMatrix2d> *",
                  index as "size_t"]
                -> * const [f64;2*3] as "const pxr::GfMatrix2d *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayMatrix2d {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayMatrix2d");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfMatrix2d> *",
                  index as "size_t"]
                -> * mut [f64;2*3] as "pxr::GfMatrix2d *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Matrix3d(pub [f64;3*3]);

impl From<&[f64;3*3]> for &Matrix3d {
    fn from(other : &[f64;3*3]) -> Self {
        unsafe { &*((other as *const [f64;3*3]) as *const Matrix3d) }
    }
}

cpp_class!(pub unsafe struct ArrayMatrix3d as "pxr::VtArray<pxr::GfMatrix3d>");

impl VtArray<[f64;3*3]> for ArrayMatrix3d {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayMatrix3d as "pxr::VtArray<pxr::GfMatrix3d>" {
                return pxr::VtArray<pxr::GfMatrix3d>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfMatrix3d> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfMatrix3d> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f64;3*3]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfMatrix3d> *",
                  elem as "const pxr::GfMatrix3d *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayMatrix3d {
    type Output = [f64;3*3];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayMatrix3d");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfMatrix3d> *",
                  index as "size_t"]
                -> * const [f64;3*3] as "const pxr::GfMatrix3d *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayMatrix3d {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayMatrix3d");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfMatrix3d> *",
                  index as "size_t"]
                -> * mut [f64;3*3] as "pxr::GfMatrix3d *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Matrix4d(pub [f64;4*4]);

impl From<&[f64;4*4]> for &Matrix4d {
    fn from(other : &[f64;4*4]) -> Self {
        unsafe { &*((other as *const [f64;4*4]) as *const Matrix4d) }
    }
}

cpp_class!(pub unsafe struct ArrayMatrix4d as "pxr::VtArray<pxr::GfMatrix4d>");

impl VtArray<[f64;4*4]> for ArrayMatrix4d {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayMatrix4d as "pxr::VtArray<pxr::GfMatrix4d>" {
                return pxr::VtArray<pxr::GfMatrix4d>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfMatrix4d> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfMatrix4d> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f64;4*4]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfMatrix4d> *",
                  elem as "const pxr::GfMatrix4d *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayMatrix4d {
    type Output = [f64;4*4];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayMatrix4d");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfMatrix4d> *",
                  index as "size_t"]
                -> * const [f64;4*4] as "const pxr::GfMatrix4d *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayMatrix4d {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayMatrix4d");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfMatrix4d> *",
                  index as "size_t"]
                -> * mut [f64;4*4] as "pxr::GfMatrix4d *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Quatd(pub [f64;4]);

impl From<&[f64;4]> for &Quatd {
    fn from(other : &[f64;4]) -> Self {
        unsafe { &*((other as *const [f64;4]) as *const Quatd) }
    }
}

cpp_class!(pub unsafe struct ArrayQuatd as "pxr::VtArray<pxr::GfQuatd>");

impl VtArray<[f64;4]> for ArrayQuatd {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayQuatd as "pxr::VtArray<pxr::GfQuatd>" {
                return pxr::VtArray<pxr::GfQuatd>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfQuatd> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfQuatd> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f64;4]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfQuatd> *",
                  elem as "const pxr::GfQuatd *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayQuatd {
    type Output = [f64;4];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayQuatd");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfQuatd> *",
                  index as "size_t"]
                -> * const [f64;4] as "const pxr::GfQuatd *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayQuatd {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayQuatd");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfQuatd> *",
                  index as "size_t"]
                -> * mut [f64;4] as "pxr::GfQuatd *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Quatf(pub [f32;4]);

impl From<&[f32;4]> for &Quatf {
    fn from(other : &[f32;4]) -> Self {
        unsafe { &*((other as *const [f32;4]) as *const Quatf) }
    }
}

cpp_class!(pub unsafe struct ArrayQuatf as "pxr::VtArray<pxr::GfQuatf>");

impl VtArray<[f32;4]> for ArrayQuatf {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayQuatf as "pxr::VtArray<pxr::GfQuatf>" {
                return pxr::VtArray<pxr::GfQuatf>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfQuatf> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfQuatf> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f32;4]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfQuatf> *",
                  elem as "const pxr::GfQuatf *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayQuatf {
    type Output = [f32;4];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayQuatf");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfQuatf> *",
                  index as "size_t"]
                -> * const [f32;4] as "const pxr::GfQuatf *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayQuatf {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayQuatf");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfQuatf> *",
                  index as "size_t"]
                -> * mut [f32;4] as "pxr::GfQuatf *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Quath(pub [f16;4]);

impl From<&[f16;4]> for &Quath {
    fn from(other : &[f16;4]) -> Self {
        unsafe { &*((other as *const [f16;4]) as *const Quath) }
    }
}

cpp_class!(pub unsafe struct ArrayQuath as "pxr::VtArray<pxr::GfQuath>");

impl VtArray<[f16;4]> for ArrayQuath {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayQuath as "pxr::VtArray<pxr::GfQuath>" {
                return pxr::VtArray<pxr::GfQuath>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfQuath> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfQuath> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f16;4]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfQuath> *",
                  elem as "const pxr::GfQuath *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayQuath {
    type Output = [f16;4];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayQuath");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfQuath> *",
                  index as "size_t"]
                -> * const [f16;4] as "const pxr::GfQuath *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayQuath {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayQuath");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfQuath> *",
                  index as "size_t"]
                -> * mut [f16;4] as "pxr::GfQuath *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec2d(pub [f64;2]);

impl From<&[f64;2]> for &Vec2d {
    fn from(other : &[f64;2]) -> Self {
        unsafe { &*((other as *const [f64;2]) as *const Vec2d) }
    }
}

cpp_class!(pub unsafe struct ArrayVec2d as "pxr::VtArray<pxr::GfVec2d>");

impl VtArray<[f64;2]> for ArrayVec2d {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec2d as "pxr::VtArray<pxr::GfVec2d>" {
                return pxr::VtArray<pxr::GfVec2d>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec2d> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2d> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f64;2]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2d> *",
                  elem as "const pxr::GfVec2d *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec2d {
    type Output = [f64;2];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec2d");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec2d> *",
                  index as "size_t"]
                -> * const [f64;2] as "const pxr::GfVec2d *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec2d {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec2d");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2d> *",
                  index as "size_t"]
                -> * mut [f64;2] as "pxr::GfVec2d *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec2f(pub [f32;2]);

impl From<&[f32;2]> for &Vec2f {
    fn from(other : &[f32;2]) -> Self {
        unsafe { &*((other as *const [f32;2]) as *const Vec2f) }
    }
}

cpp_class!(pub unsafe struct ArrayVec2f as "pxr::VtArray<pxr::GfVec2f>");

impl VtArray<[f32;2]> for ArrayVec2f {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec2f as "pxr::VtArray<pxr::GfVec2f>" {
                return pxr::VtArray<pxr::GfVec2f>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec2f> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2f> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f32;2]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2f> *",
                  elem as "const pxr::GfVec2f *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec2f {
    type Output = [f32;2];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec2f");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec2f> *",
                  index as "size_t"]
                -> * const [f32;2] as "const pxr::GfVec2f *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec2f {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec2f");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2f> *",
                  index as "size_t"]
                -> * mut [f32;2] as "pxr::GfVec2f *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec2h(pub [f16;2]);

impl From<&[f16;2]> for &Vec2h {
    fn from(other : &[f16;2]) -> Self {
        unsafe { &*((other as *const [f16;2]) as *const Vec2h) }
    }
}

cpp_class!(pub unsafe struct ArrayVec2h as "pxr::VtArray<pxr::GfVec2h>");

impl VtArray<[f16;2]> for ArrayVec2h {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec2h as "pxr::VtArray<pxr::GfVec2h>" {
                return pxr::VtArray<pxr::GfVec2h>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec2h> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2h> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f16;2]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2h> *",
                  elem as "const pxr::GfVec2h *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec2h {
    type Output = [f16;2];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec2h");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec2h> *",
                  index as "size_t"]
                -> * const [f16;2] as "const pxr::GfVec2h *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec2h {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec2h");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2h> *",
                  index as "size_t"]
                -> * mut [f16;2] as "pxr::GfVec2h *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec2i(pub [i32;2]);

impl From<&[i32;2]> for &Vec2i {
    fn from(other : &[i32;2]) -> Self {
        unsafe { &*((other as *const [i32;2]) as *const Vec2i) }
    }
}

cpp_class!(pub unsafe struct ArrayVec2i as "pxr::VtArray<pxr::GfVec2i>");

impl VtArray<[i32;2]> for ArrayVec2i {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec2i as "pxr::VtArray<pxr::GfVec2i>" {
                return pxr::VtArray<pxr::GfVec2i>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec2i> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2i> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[i32;2]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2i> *",
                  elem as "const pxr::GfVec2i *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec2i {
    type Output = [i32;2];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec2i");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec2i> *",
                  index as "size_t"]
                -> * const [i32;2] as "const pxr::GfVec2i *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec2i {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec2i");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec2i> *",
                  index as "size_t"]
                -> * mut [i32;2] as "pxr::GfVec2i *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec3d(pub [f64;3]);

impl From<&[f64;3]> for &Vec3d {
    fn from(other : &[f64;3]) -> Self {
        unsafe { &*((other as *const [f64;3]) as *const Vec3d) }
    }
}

cpp_class!(pub unsafe struct ArrayVec3d as "pxr::VtArray<pxr::GfVec3d>");

impl VtArray<[f64;3]> for ArrayVec3d {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec3d as "pxr::VtArray<pxr::GfVec3d>" {
                return pxr::VtArray<pxr::GfVec3d>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec3d> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3d> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f64;3]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3d> *",
                  elem as "const pxr::GfVec3d *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec3d {
    type Output = [f64;3];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec3d");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec3d> *",
                  index as "size_t"]
                -> * const [f64;3] as "const pxr::GfVec3d *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec3d {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec3d");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3d> *",
                  index as "size_t"]
                -> * mut [f64;3] as "pxr::GfVec3d *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec3f(pub [f32;3]);

impl From<&[f32;3]> for &Vec3f {
    fn from(other : &[f32;3]) -> Self {
        unsafe { &*((other as *const [f32;3]) as *const Vec3f) }
    }
}

cpp_class!(pub unsafe struct ArrayVec3f as "pxr::VtArray<pxr::GfVec3f>");

impl VtArray<[f32;3]> for ArrayVec3f {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec3f as "pxr::VtArray<pxr::GfVec3f>" {
                return pxr::VtArray<pxr::GfVec3f>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec3f> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3f> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f32;3]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3f> *",
                  elem as "const pxr::GfVec3f *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec3f {
    type Output = [f32;3];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec3f");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec3f> *",
                  index as "size_t"]
                -> * const [f32;3] as "const pxr::GfVec3f *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec3f {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec3f");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3f> *",
                  index as "size_t"]
                -> * mut [f32;3] as "pxr::GfVec3f *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec3h(pub [f16;3]);

impl From<&[f16;3]> for &Vec3h {
    fn from(other : &[f16;3]) -> Self {
        unsafe { &*((other as *const [f16;3]) as *const Vec3h) }
    }
}

cpp_class!(pub unsafe struct ArrayVec3h as "pxr::VtArray<pxr::GfVec3h>");

impl VtArray<[f16;3]> for ArrayVec3h {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec3h as "pxr::VtArray<pxr::GfVec3h>" {
                return pxr::VtArray<pxr::GfVec3h>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec3h> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3h> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f16;3]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3h> *",
                  elem as "const pxr::GfVec3h *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec3h {
    type Output = [f16;3];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec3h");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec3h> *",
                  index as "size_t"]
                -> * const [f16;3] as "const pxr::GfVec3h *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec3h {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec3h");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3h> *",
                  index as "size_t"]
                -> * mut [f16;3] as "pxr::GfVec3h *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec3i(pub [i32;3]);

impl From<&[i32;3]> for &Vec3i {
    fn from(other : &[i32;3]) -> Self {
        unsafe { &*((other as *const [i32;3]) as *const Vec3i) }
    }
}

cpp_class!(pub unsafe struct ArrayVec3i as "pxr::VtArray<pxr::GfVec3i>");

impl VtArray<[i32;3]> for ArrayVec3i {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec3i as "pxr::VtArray<pxr::GfVec3i>" {
                return pxr::VtArray<pxr::GfVec3i>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec3i> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3i> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[i32;3]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3i> *",
                  elem as "const pxr::GfVec3i *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec3i {
    type Output = [i32;3];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec3i");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec3i> *",
                  index as "size_t"]
                -> * const [i32;3] as "const pxr::GfVec3i *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec3i {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec3i");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec3i> *",
                  index as "size_t"]
                -> * mut [i32;3] as "pxr::GfVec3i *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec4d(pub [f64;4]);

impl From<&[f64;4]> for &Vec4d {
    fn from(other : &[f64;4]) -> Self {
        unsafe { &*((other as *const [f64;4]) as *const Vec4d) }
    }
}

cpp_class!(pub unsafe struct ArrayVec4d as "pxr::VtArray<pxr::GfVec4d>");

impl VtArray<[f64;4]> for ArrayVec4d {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec4d as "pxr::VtArray<pxr::GfVec4d>" {
                return pxr::VtArray<pxr::GfVec4d>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec4d> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4d> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f64;4]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4d> *",
                  elem as "const pxr::GfVec4d *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec4d {
    type Output = [f64;4];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec4d");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec4d> *",
                  index as "size_t"]
                -> * const [f64;4] as "const pxr::GfVec4d *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec4d {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec4d");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4d> *",
                  index as "size_t"]
                -> * mut [f64;4] as "pxr::GfVec4d *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec4f(pub [f32;4]);

impl From<&[f32;4]> for &Vec4f {
    fn from(other : &[f32;4]) -> Self {
        unsafe { &*((other as *const [f32;4]) as *const Vec4f) }
    }
}

cpp_class!(pub unsafe struct ArrayVec4f as "pxr::VtArray<pxr::GfVec4f>");

impl VtArray<[f32;4]> for ArrayVec4f {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec4f as "pxr::VtArray<pxr::GfVec4f>" {
                return pxr::VtArray<pxr::GfVec4f>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec4f> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4f> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f32;4]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4f> *",
                  elem as "const pxr::GfVec4f *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec4f {
    type Output = [f32;4];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec4f");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec4f> *",
                  index as "size_t"]
                -> * const [f32;4] as "const pxr::GfVec4f *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec4f {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec4f");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4f> *",
                  index as "size_t"]
                -> * mut [f32;4] as "pxr::GfVec4f *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec4h(pub [f16;4]);

impl From<&[f16;4]> for &Vec4h {
    fn from(other : &[f16;4]) -> Self {
        unsafe { &*((other as *const [f16;4]) as *const Vec4h) }
    }
}

cpp_class!(pub unsafe struct ArrayVec4h as "pxr::VtArray<pxr::GfVec4h>");

impl VtArray<[f16;4]> for ArrayVec4h {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec4h as "pxr::VtArray<pxr::GfVec4h>" {
                return pxr::VtArray<pxr::GfVec4h>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec4h> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4h> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[f16;4]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4h> *",
                  elem as "const pxr::GfVec4h *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec4h {
    type Output = [f16;4];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec4h");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec4h> *",
                  index as "size_t"]
                -> * const [f16;4] as "const pxr::GfVec4h *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec4h {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec4h");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4h> *",
                  index as "size_t"]
                -> * mut [f16;4] as "pxr::GfVec4h *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}

#[repr(transparent)]
pub struct Vec4i(pub [i32;4]);

impl From<&[i32;4]> for &Vec4i {
    fn from(other : &[i32;4]) -> Self {
        unsafe { &*((other as *const [i32;4]) as *const Vec4i) }
    }
}

cpp_class!(pub unsafe struct ArrayVec4i as "pxr::VtArray<pxr::GfVec4i>");

impl VtArray<[i32;4]> for ArrayVec4i {
    fn new() -> Self {
        unsafe {
            cpp!([] -> ArrayVec4i as "pxr::VtArray<pxr::GfVec4i>" {
                return pxr::VtArray<pxr::GfVec4i>();
            })
        }
    }

    fn boxed() -> std::boxed::Box<Self> {
        std::boxed::Box::new(Self::new())
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec4i> *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    fn reserve(& mut self, num : usize) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4i> *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    fn push_back(& mut self, elem : &[i32;4]) {
        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4i> *",
                  elem as "const pxr::GfVec4i *"] {
                self->push_back(*elem);
            })
        }
    }
}

impl std::ops::Index<usize> for ArrayVec4i {
    type Output = [i32;4];
    fn index(&self, index: usize) -> &Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec4i");
        }

        unsafe {
            cpp!([self as "const pxr::VtArray<pxr::GfVec4i> *",
                  index as "size_t"]
                -> * const [i32;4] as "const pxr::GfVec4i *" {
                return &self->operator[](index);
            })
            .as_ref()
            .expect("Error converting pointer to reference")
        }
    }
}

impl std::ops::IndexMut<usize> for ArrayVec4i {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        // Bounds check
        if index >= self.size() {
            panic!("Out of bounds VtArray access for ArrayVec4i");
        }

        unsafe {
            cpp!([self as "pxr::VtArray<pxr::GfVec4i> *",
                  index as "size_t"]
                -> * mut [i32;4] as "pxr::GfVec4i *" {
                return &self->operator[](index);
            })
            .as_mut()
            .expect("Error converting pointer to reference")
        }
    }
}



cpp! {{
    #include <string>

    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/base/vt/value.h"
    #include "pxr/base/vt/array.h"
    #include "pxr/base/gf/half.h"
    #include "pxr/usd/sdf/timeCode.h"
    #include "pxr/base/tf/token.h"
    #include "pxr/usd/sdf/assetPath.h"
    #include "pxr/base/gf/matrix2d.h"
    #include "pxr/base/gf/matrix3d.h"
    #include "pxr/base/gf/matrix4d.h"
    #include "pxr/base/gf/quatd.h"
    #include "pxr/base/gf/quatf.h"
    #include "pxr/base/gf/quath.h"
    #include "pxr/base/gf/vec2d.h"
    #include "pxr/base/gf/vec2f.h"
    #include "pxr/base/gf/vec2h.h"
    #include "pxr/base/gf/vec2i.h"
    #include "pxr/base/gf/vec3d.h"
    #include "pxr/base/gf/vec3f.h"
    #include "pxr/base/gf/vec3h.h"
    #include "pxr/base/gf/vec3i.h"
    #include "pxr/base/gf/vec4d.h"
    #include "pxr/base/gf/vec4f.h"
    #include "pxr/base/gf/vec4h.h"
    #include "pxr/base/gf/vec4i.h"

    #pragma GCC diagnostic pop
}}

// The String type is written by hand because:
// - The constructor for VtValue takes a pointer to "const char *", but internally
//  stores a std::string.
// - CStr::as_ptr has to be called to convert to a "const char *".
// - CStr::from has to be called to convert from a "const char *".
//
// Why did we choose CStr to get/set string vt::Values ?
//
// The options available are:
//  - &std::string::str
//  - std::string::String
//  - std::ffi::CString,
//  - std::ffi::CStr
//  - * const std::os::raw::c_char;
//
// The standard rust str and String types are for unicode strings, moving to and
// from them requires a test for valid unicode. This goes against the low
// overhead goal of this binding.
//
// Converting to and from * const std::os::raw::c_char has basically no cost.
// But you need an unsafe block to do anything useful with pointers, and this
// goes against the primary goal of this binding, which is to be a safe
// api.
//
// CString/CStr provides a string type that matches c strings, complete with a
// null terminator. So we can ensure converting them to 'const char *' will have
// very little cost. CString is the owned representation (like std::string), while
// CStr is the reference representation.
//
// There is a cost to using CStr. That is, the API user has to do the conversion
// from &str to &CStr, but by pushing these upwards, there is more opportunity
// to reduce the number of times the conversions need to be done.

pub trait VtArray<T> {
    fn new() -> Self;
    fn boxed() -> std::boxed::Box<Self>;
    fn size(&self) -> usize;
    fn reserve(& mut self, num : usize);
    fn push_back(& mut self, elem : &T);
}

#[repr(transparent)]
pub struct String(pub std::ffi::CStr);

impl From<&std::ffi::CStr> for &String {
    fn from(other : &std::ffi::CStr) -> Self {
        unsafe { &*((other as *const std::ffi::CStr) as *const String) }
    }
}

impl From<&String> for Value {
    fn from(other: &String) -> Self {
        let c_char = other.0.as_ptr();
        unsafe {
            cpp!([c_char as "const char *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(c_char);
            })
        }
    }
}

impl AsRef<String> for Value {
    fn as_ref(&self) -> &String {
        use std::os::raw::c_char;

        <&String>::from(
            unsafe {
                std::ffi::CStr::from_ptr(
                
                    cpp!([self as "const pxr::VtValue *"] ->  * const c_char as "const char *" {
                        return self->Get<std::string>().c_str();
                    })

                )
            }
        )
    }
}



// Scalar
impl From<&Bool> for Value {
    fn from(other: &Bool) -> Self {
        unsafe {
            cpp!([other as "const bool *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Bool> for Value {
    fn as_ref(&self) -> &Bool {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Bool as "const bool *" {
                return &(self->Get<bool>());
            })
        }
    }
}

// Array
impl From<&ArrayBool> for Value {
    fn from(other: &ArrayBool) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<bool> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayBool> for Value {
    fn as_ref(&self) -> &ArrayBool {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayBool as "const pxr::VtArray<bool> *" {
                return &(self->Get<pxr::VtArray<bool>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&UChar> for Value {
    fn from(other: &UChar) -> Self {
        unsafe {
            cpp!([other as "const uint8_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<UChar> for Value {
    fn as_ref(&self) -> &UChar {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &UChar as "const uint8_t *" {
                return &(self->Get<uint8_t>());
            })
        }
    }
}

// Array
impl From<&ArrayUChar> for Value {
    fn from(other: &ArrayUChar) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<uint8_t> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayUChar> for Value {
    fn as_ref(&self) -> &ArrayUChar {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayUChar as "const pxr::VtArray<uint8_t> *" {
                return &(self->Get<pxr::VtArray<uint8_t>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Int> for Value {
    fn from(other: &Int) -> Self {
        unsafe {
            cpp!([other as "const int32_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Int> for Value {
    fn as_ref(&self) -> &Int {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Int as "const int32_t *" {
                return &(self->Get<int32_t>());
            })
        }
    }
}

// Array
impl From<&ArrayInt> for Value {
    fn from(other: &ArrayInt) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<int32_t> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayInt> for Value {
    fn as_ref(&self) -> &ArrayInt {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayInt as "const pxr::VtArray<int32_t> *" {
                return &(self->Get<pxr::VtArray<int32_t>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&UInt> for Value {
    fn from(other: &UInt) -> Self {
        unsafe {
            cpp!([other as "const uint32_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<UInt> for Value {
    fn as_ref(&self) -> &UInt {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &UInt as "const uint32_t *" {
                return &(self->Get<uint32_t>());
            })
        }
    }
}

// Array
impl From<&ArrayUInt> for Value {
    fn from(other: &ArrayUInt) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<uint32_t> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayUInt> for Value {
    fn as_ref(&self) -> &ArrayUInt {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayUInt as "const pxr::VtArray<uint32_t> *" {
                return &(self->Get<pxr::VtArray<uint32_t>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Int64> for Value {
    fn from(other: &Int64) -> Self {
        unsafe {
            cpp!([other as "const int64_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Int64> for Value {
    fn as_ref(&self) -> &Int64 {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Int64 as "const int64_t *" {
                return &(self->Get<int64_t>());
            })
        }
    }
}

// Array
impl From<&ArrayInt64> for Value {
    fn from(other: &ArrayInt64) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<int64_t> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayInt64> for Value {
    fn as_ref(&self) -> &ArrayInt64 {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayInt64 as "const pxr::VtArray<int64_t> *" {
                return &(self->Get<pxr::VtArray<int64_t>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&UInt64> for Value {
    fn from(other: &UInt64) -> Self {
        unsafe {
            cpp!([other as "const uint64_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<UInt64> for Value {
    fn as_ref(&self) -> &UInt64 {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &UInt64 as "const uint64_t *" {
                return &(self->Get<uint64_t>());
            })
        }
    }
}

// Array
impl From<&ArrayUInt64> for Value {
    fn from(other: &ArrayUInt64) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<uint64_t> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayUInt64> for Value {
    fn as_ref(&self) -> &ArrayUInt64 {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayUInt64 as "const pxr::VtArray<uint64_t> *" {
                return &(self->Get<pxr::VtArray<uint64_t>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Half> for Value {
    fn from(other: &Half) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfHalf *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Half> for Value {
    fn as_ref(&self) -> &Half {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Half as "const pxr::GfHalf *" {
                return &(self->Get<pxr::GfHalf>());
            })
        }
    }
}

// Array
impl From<&ArrayHalf> for Value {
    fn from(other: &ArrayHalf) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfHalf> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayHalf> for Value {
    fn as_ref(&self) -> &ArrayHalf {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayHalf as "const pxr::VtArray<pxr::GfHalf> *" {
                return &(self->Get<pxr::VtArray<pxr::GfHalf>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Float> for Value {
    fn from(other: &Float) -> Self {
        unsafe {
            cpp!([other as "const float *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Float> for Value {
    fn as_ref(&self) -> &Float {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Float as "const float *" {
                return &(self->Get<float>());
            })
        }
    }
}

// Array
impl From<&ArrayFloat> for Value {
    fn from(other: &ArrayFloat) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<float> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayFloat> for Value {
    fn as_ref(&self) -> &ArrayFloat {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayFloat as "const pxr::VtArray<float> *" {
                return &(self->Get<pxr::VtArray<float>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Double> for Value {
    fn from(other: &Double) -> Self {
        unsafe {
            cpp!([other as "const double *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Double> for Value {
    fn as_ref(&self) -> &Double {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Double as "const double *" {
                return &(self->Get<double>());
            })
        }
    }
}

// Array
impl From<&ArrayDouble> for Value {
    fn from(other: &ArrayDouble) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<double> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayDouble> for Value {
    fn as_ref(&self) -> &ArrayDouble {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayDouble as "const pxr::VtArray<double> *" {
                return &(self->Get<pxr::VtArray<double>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&TimeCode> for Value {
    fn from(other: &TimeCode) -> Self {
        unsafe {
            cpp!([other as "const pxr::SdfTimeCode *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<TimeCode> for Value {
    fn as_ref(&self) -> &TimeCode {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &TimeCode as "const pxr::SdfTimeCode *" {
                return &(self->Get<pxr::SdfTimeCode>());
            })
        }
    }
}

// Array
impl From<&ArrayTimeCode> for Value {
    fn from(other: &ArrayTimeCode) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::SdfTimeCode> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayTimeCode> for Value {
    fn as_ref(&self) -> &ArrayTimeCode {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayTimeCode as "const pxr::VtArray<pxr::SdfTimeCode> *" {
                return &(self->Get<pxr::VtArray<pxr::SdfTimeCode>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Token> for Value {
    fn from(other: &Token) -> Self {
        unsafe {
            cpp!([other as "const pxr::TfToken *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Token> for Value {
    fn as_ref(&self) -> &Token {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Token as "const pxr::TfToken *" {
                return &(self->Get<pxr::TfToken>());
            })
        }
    }
}

// Array
impl From<&ArrayToken> for Value {
    fn from(other: &ArrayToken) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::TfToken> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayToken> for Value {
    fn as_ref(&self) -> &ArrayToken {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayToken as "const pxr::VtArray<pxr::TfToken> *" {
                return &(self->Get<pxr::VtArray<pxr::TfToken>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Asset> for Value {
    fn from(other: &Asset) -> Self {
        unsafe {
            cpp!([other as "const pxr::SdfAssetPath *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Asset> for Value {
    fn as_ref(&self) -> &Asset {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Asset as "const pxr::SdfAssetPath *" {
                return &(self->Get<pxr::SdfAssetPath>());
            })
        }
    }
}

// Array
impl From<&ArrayAsset> for Value {
    fn from(other: &ArrayAsset) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::SdfAssetPath> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayAsset> for Value {
    fn as_ref(&self) -> &ArrayAsset {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayAsset as "const pxr::VtArray<pxr::SdfAssetPath> *" {
                return &(self->Get<pxr::VtArray<pxr::SdfAssetPath>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Matrix2d> for Value {
    fn from(other: &Matrix2d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfMatrix2d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Matrix2d> for Value {
    fn as_ref(&self) -> &Matrix2d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Matrix2d as "const pxr::GfMatrix2d *" {
                return &(self->Get<pxr::GfMatrix2d>());
            })
        }
    }
}

// Array
impl From<&ArrayMatrix2d> for Value {
    fn from(other: &ArrayMatrix2d) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfMatrix2d> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayMatrix2d> for Value {
    fn as_ref(&self) -> &ArrayMatrix2d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayMatrix2d as "const pxr::VtArray<pxr::GfMatrix2d> *" {
                return &(self->Get<pxr::VtArray<pxr::GfMatrix2d>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Matrix3d> for Value {
    fn from(other: &Matrix3d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfMatrix3d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Matrix3d> for Value {
    fn as_ref(&self) -> &Matrix3d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Matrix3d as "const pxr::GfMatrix3d *" {
                return &(self->Get<pxr::GfMatrix3d>());
            })
        }
    }
}

// Array
impl From<&ArrayMatrix3d> for Value {
    fn from(other: &ArrayMatrix3d) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfMatrix3d> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayMatrix3d> for Value {
    fn as_ref(&self) -> &ArrayMatrix3d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayMatrix3d as "const pxr::VtArray<pxr::GfMatrix3d> *" {
                return &(self->Get<pxr::VtArray<pxr::GfMatrix3d>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Matrix4d> for Value {
    fn from(other: &Matrix4d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfMatrix4d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Matrix4d> for Value {
    fn as_ref(&self) -> &Matrix4d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Matrix4d as "const pxr::GfMatrix4d *" {
                return &(self->Get<pxr::GfMatrix4d>());
            })
        }
    }
}

// Array
impl From<&ArrayMatrix4d> for Value {
    fn from(other: &ArrayMatrix4d) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfMatrix4d> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayMatrix4d> for Value {
    fn as_ref(&self) -> &ArrayMatrix4d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayMatrix4d as "const pxr::VtArray<pxr::GfMatrix4d> *" {
                return &(self->Get<pxr::VtArray<pxr::GfMatrix4d>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Quatd> for Value {
    fn from(other: &Quatd) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfQuatd *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Quatd> for Value {
    fn as_ref(&self) -> &Quatd {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Quatd as "const pxr::GfQuatd *" {
                return &(self->Get<pxr::GfQuatd>());
            })
        }
    }
}

// Array
impl From<&ArrayQuatd> for Value {
    fn from(other: &ArrayQuatd) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfQuatd> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayQuatd> for Value {
    fn as_ref(&self) -> &ArrayQuatd {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayQuatd as "const pxr::VtArray<pxr::GfQuatd> *" {
                return &(self->Get<pxr::VtArray<pxr::GfQuatd>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Quatf> for Value {
    fn from(other: &Quatf) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfQuatf *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Quatf> for Value {
    fn as_ref(&self) -> &Quatf {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Quatf as "const pxr::GfQuatf *" {
                return &(self->Get<pxr::GfQuatf>());
            })
        }
    }
}

// Array
impl From<&ArrayQuatf> for Value {
    fn from(other: &ArrayQuatf) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfQuatf> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayQuatf> for Value {
    fn as_ref(&self) -> &ArrayQuatf {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayQuatf as "const pxr::VtArray<pxr::GfQuatf> *" {
                return &(self->Get<pxr::VtArray<pxr::GfQuatf>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Quath> for Value {
    fn from(other: &Quath) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfQuath *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Quath> for Value {
    fn as_ref(&self) -> &Quath {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Quath as "const pxr::GfQuath *" {
                return &(self->Get<pxr::GfQuath>());
            })
        }
    }
}

// Array
impl From<&ArrayQuath> for Value {
    fn from(other: &ArrayQuath) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfQuath> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayQuath> for Value {
    fn as_ref(&self) -> &ArrayQuath {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayQuath as "const pxr::VtArray<pxr::GfQuath> *" {
                return &(self->Get<pxr::VtArray<pxr::GfQuath>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec2d> for Value {
    fn from(other: &Vec2d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec2d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec2d> for Value {
    fn as_ref(&self) -> &Vec2d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec2d as "const pxr::GfVec2d *" {
                return &(self->Get<pxr::GfVec2d>());
            })
        }
    }
}

// Array
impl From<&ArrayVec2d> for Value {
    fn from(other: &ArrayVec2d) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec2d> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec2d> for Value {
    fn as_ref(&self) -> &ArrayVec2d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec2d as "const pxr::VtArray<pxr::GfVec2d> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec2d>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec2f> for Value {
    fn from(other: &Vec2f) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec2f *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec2f> for Value {
    fn as_ref(&self) -> &Vec2f {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec2f as "const pxr::GfVec2f *" {
                return &(self->Get<pxr::GfVec2f>());
            })
        }
    }
}

// Array
impl From<&ArrayVec2f> for Value {
    fn from(other: &ArrayVec2f) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec2f> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec2f> for Value {
    fn as_ref(&self) -> &ArrayVec2f {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec2f as "const pxr::VtArray<pxr::GfVec2f> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec2f>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec2h> for Value {
    fn from(other: &Vec2h) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec2h *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec2h> for Value {
    fn as_ref(&self) -> &Vec2h {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec2h as "const pxr::GfVec2h *" {
                return &(self->Get<pxr::GfVec2h>());
            })
        }
    }
}

// Array
impl From<&ArrayVec2h> for Value {
    fn from(other: &ArrayVec2h) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec2h> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec2h> for Value {
    fn as_ref(&self) -> &ArrayVec2h {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec2h as "const pxr::VtArray<pxr::GfVec2h> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec2h>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec2i> for Value {
    fn from(other: &Vec2i) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec2i *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec2i> for Value {
    fn as_ref(&self) -> &Vec2i {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec2i as "const pxr::GfVec2i *" {
                return &(self->Get<pxr::GfVec2i>());
            })
        }
    }
}

// Array
impl From<&ArrayVec2i> for Value {
    fn from(other: &ArrayVec2i) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec2i> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec2i> for Value {
    fn as_ref(&self) -> &ArrayVec2i {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec2i as "const pxr::VtArray<pxr::GfVec2i> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec2i>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec3d> for Value {
    fn from(other: &Vec3d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec3d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec3d> for Value {
    fn as_ref(&self) -> &Vec3d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec3d as "const pxr::GfVec3d *" {
                return &(self->Get<pxr::GfVec3d>());
            })
        }
    }
}

// Array
impl From<&ArrayVec3d> for Value {
    fn from(other: &ArrayVec3d) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec3d> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec3d> for Value {
    fn as_ref(&self) -> &ArrayVec3d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec3d as "const pxr::VtArray<pxr::GfVec3d> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec3d>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec3f> for Value {
    fn from(other: &Vec3f) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec3f *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec3f> for Value {
    fn as_ref(&self) -> &Vec3f {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec3f as "const pxr::GfVec3f *" {
                return &(self->Get<pxr::GfVec3f>());
            })
        }
    }
}

// Array
impl From<&ArrayVec3f> for Value {
    fn from(other: &ArrayVec3f) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec3f> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec3f> for Value {
    fn as_ref(&self) -> &ArrayVec3f {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec3f as "const pxr::VtArray<pxr::GfVec3f> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec3f>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec3h> for Value {
    fn from(other: &Vec3h) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec3h *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec3h> for Value {
    fn as_ref(&self) -> &Vec3h {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec3h as "const pxr::GfVec3h *" {
                return &(self->Get<pxr::GfVec3h>());
            })
        }
    }
}

// Array
impl From<&ArrayVec3h> for Value {
    fn from(other: &ArrayVec3h) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec3h> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec3h> for Value {
    fn as_ref(&self) -> &ArrayVec3h {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec3h as "const pxr::VtArray<pxr::GfVec3h> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec3h>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec3i> for Value {
    fn from(other: &Vec3i) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec3i *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec3i> for Value {
    fn as_ref(&self) -> &Vec3i {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec3i as "const pxr::GfVec3i *" {
                return &(self->Get<pxr::GfVec3i>());
            })
        }
    }
}

// Array
impl From<&ArrayVec3i> for Value {
    fn from(other: &ArrayVec3i) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec3i> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec3i> for Value {
    fn as_ref(&self) -> &ArrayVec3i {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec3i as "const pxr::VtArray<pxr::GfVec3i> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec3i>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec4d> for Value {
    fn from(other: &Vec4d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec4d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec4d> for Value {
    fn as_ref(&self) -> &Vec4d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec4d as "const pxr::GfVec4d *" {
                return &(self->Get<pxr::GfVec4d>());
            })
        }
    }
}

// Array
impl From<&ArrayVec4d> for Value {
    fn from(other: &ArrayVec4d) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec4d> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec4d> for Value {
    fn as_ref(&self) -> &ArrayVec4d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec4d as "const pxr::VtArray<pxr::GfVec4d> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec4d>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec4f> for Value {
    fn from(other: &Vec4f) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec4f *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec4f> for Value {
    fn as_ref(&self) -> &Vec4f {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec4f as "const pxr::GfVec4f *" {
                return &(self->Get<pxr::GfVec4f>());
            })
        }
    }
}

// Array
impl From<&ArrayVec4f> for Value {
    fn from(other: &ArrayVec4f) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec4f> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec4f> for Value {
    fn as_ref(&self) -> &ArrayVec4f {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec4f as "const pxr::VtArray<pxr::GfVec4f> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec4f>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec4h> for Value {
    fn from(other: &Vec4h) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec4h *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec4h> for Value {
    fn as_ref(&self) -> &Vec4h {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec4h as "const pxr::GfVec4h *" {
                return &(self->Get<pxr::GfVec4h>());
            })
        }
    }
}

// Array
impl From<&ArrayVec4h> for Value {
    fn from(other: &ArrayVec4h) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec4h> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec4h> for Value {
    fn as_ref(&self) -> &ArrayVec4h {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec4h as "const pxr::VtArray<pxr::GfVec4h> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec4h>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}



// Scalar
impl From<&Vec4i> for Value {
    fn from(other: &Vec4i) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec4i *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec4i> for Value {
    fn as_ref(&self) -> &Vec4i {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec4i as "const pxr::GfVec4i *" {
                return &(self->Get<pxr::GfVec4i>());
            })
        }
    }
}

// Array
impl From<&ArrayVec4i> for Value {
    fn from(other: &ArrayVec4i) -> Self {
        unsafe {
            cpp!([other as "const pxr::VtArray<pxr::GfVec4i> *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<ArrayVec4i> for Value {
    fn as_ref(&self) -> &ArrayVec4i {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> * const ArrayVec4i as "const pxr::VtArray<pxr::GfVec4i> *" {
                return &(self->Get<pxr::VtArray<pxr::GfVec4i>>());
            }).as_ref().expect("Error converting from pointer to reference")
        }
    }
}


