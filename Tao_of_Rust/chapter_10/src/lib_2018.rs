pub mod outer_mod {
    pub(self) fn outer_mod_fn() {}
    pub mod inner_mod {
        // 在 Rust2018 模块系统中必须使用use导入
        use crate::outer_mod::outer_mod_fn;
        // 对外层模块 `outer_mod` 可见
        pub(in crate::outer_mod) fn outer_mod_visible_fn() {}
        // 在 `outer_mod` 内部可见
        pub(super) fn super_mod_visible_fn() {
            // 访问同一个模块的函数
            inner_mod_visible_fn();
            // use 导入了 out_mod
            outer_mod_fn();
        }
        // 仅在 'inner_mod' 内部可见
        pub(self) fn inner_mod_visible_fn() {}
    }
    pub fn foo() {
        inner_mod::outer_mod_visible_fn();
        inner_mod::create_visible_fn();
        inner_mod::super_mod_visible_fn();
        // inner_mod::inner_mod_visible_fn();
    }
}
fn bar() {
    outer_mod::inner_mod::crate_visible_fn();
    // 只对 `out_mod` 可见
    // outer_mod::inner_mod::super_mod_visible_fn();
    // 只对 `outer_mod` 可见
    // outer_mod::inner_mod::outer_mod_visible_fn();
    // 通过 foo 函数调用
    out_mod::foo();
}
