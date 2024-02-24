fn main() {
    let my_struct = MyStruct;

    // 使用 CombinedTrait 作为 trait object
    let trait_object: &(dyn CombinedTrait) = &my_struct;

    trait_object.method1();
    trait_object.method2();
}
