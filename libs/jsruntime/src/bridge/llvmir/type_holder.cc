// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/jsruntime/src/bridge/llvmir/type_holder.cc.njk

#include "type_holder.hh"

#include <climits>

namespace {
constexpr unsigned kWorkBits = sizeof(size_t) * CHAR_BIT;
}

llvm::Type* TypeHolder::GetWordType() {
  return builder_.getIntNTy(kWorkBits);
}

llvm::Value* TypeHolder::GetWord(size_t value) {
  return builder_.getIntN(kWorkBits, value);
}

llvm::StructType* TypeHolder::CreateValueType() {
  if (value_type_ == nullptr) {
    value_type_ = llvm::StructType::create(context_, "Value");
    value_type_->setBody({
        // kind
        builder_.getInt8Ty(),
        // holder
        builder_.getInt64Ty(),
    });
  }
  return value_type_;
}

llvm::StructType* TypeHolder::CreateBindingType() {
  if (binding_type_ == nullptr) {
    binding_type_ = llvm::StructType::create(context_, "Binding");
    binding_type_->setBody({
        // kind
        builder_.getInt8Ty(),
        // flags
        builder_.getInt8Ty(),
        // reserved
        builder_.getInt16Ty(),
        // symbol
        builder_.getInt32Ty(),
        // holder
        builder_.getInt64Ty(),
    });
  }
  return binding_type_;
}

llvm::FunctionType* TypeHolder::CreateFunctionType() {
  if (function_type_ == nullptr) {
    function_type_ = llvm::FunctionType::get(
        // status code
        builder_.getInt32Ty(),
        {
            // runtime (pointer to the runtime)
            builder_.getPtrTy(),
            // outer (pointer to the outer function scope)
            builder_.getPtrTy(),
            // argc
            GetWordType(),
            // argv (pointer to a list of bindings)
            builder_.getPtrTy(),
            // return value (pointer to a value)
            builder_.getPtrTy(),
        },
        false);
  }
  return function_type_;
}

llvm::Function* TypeHolder::CreateRuntimeToBoolean() {
  if (runtime_to_boolean_ == nullptr) {
    auto* prototype = llvm::FunctionType::get(
        builder_.getInt1Ty(), {builder_.getPtrTy(), builder_.getPtrTy()}, false);
    runtime_to_boolean_ = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_to_boolean", module_);
  }
  return runtime_to_boolean_;
}

llvm::Function* TypeHolder::CreateRuntimeToNumeric() {
  if (runtime_to_numeric_ == nullptr) {
    auto* prototype = llvm::FunctionType::get(
        builder_.getDoubleTy(), {builder_.getPtrTy(), builder_.getPtrTy()}, false);
    runtime_to_numeric_ = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_to_numeric", module_);
  }
  return runtime_to_numeric_;
}

llvm::Function* TypeHolder::CreateRuntimeToInt32() {
  if (runtime_to_int32_ == nullptr) {
    auto* prototype = llvm::FunctionType::get(
        builder_.getInt32Ty(), {builder_.getPtrTy(), builder_.getDoubleTy()}, false);
    runtime_to_int32_ = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_to_int32", module_);
  }
  return runtime_to_int32_;
}

llvm::Function* TypeHolder::CreateRuntimeToUint32() {
  if (runtime_to_uint32_ == nullptr) {
    auto* prototype = llvm::FunctionType::get(
        builder_.getInt32Ty(), {builder_.getPtrTy(), builder_.getDoubleTy()}, false);
    runtime_to_uint32_ = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_to_uint32", module_);
  }
  return runtime_to_uint32_;
}

llvm::Function* TypeHolder::CreateRuntimeIsLooselyEqual() {
  if (runtime_is_loosely_equal_ == nullptr) {
    auto* prototype = llvm::FunctionType::get(builder_.getInt1Ty(),
        {builder_.getPtrTy(), builder_.getPtrTy(), builder_.getPtrTy()}, false);
    runtime_is_loosely_equal_ = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_is_loosely_equal", module_);
  }
  return runtime_is_loosely_equal_;
}

llvm::Function* TypeHolder::CreateRuntimeIsStrictlyEqual() {
  if (runtime_is_strictly_equal_ == nullptr) {
    auto* prototype = llvm::FunctionType::get(builder_.getInt1Ty(),
        {builder_.getPtrTy(), builder_.getPtrTy(), builder_.getPtrTy()}, false);
    runtime_is_strictly_equal_ = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_is_strictly_equal", module_);
  }
  return runtime_is_strictly_equal_;
}
