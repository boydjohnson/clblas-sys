use clblas_sys::{clblasSetup, clblasStatus__clblasSuccess, clblasTeardown};

#[test]
fn test_setup_teardown() {
    unsafe {
        assert_eq!(clblasSetup(), clblasStatus__clblasSuccess);
        clblasTeardown();
    }
}
