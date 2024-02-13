init();

async function init() {
    const [{WebChart, hook_panic_handler, test_method, default: init}, {main, setup}] = await Promise.all([
        import("../pkg/haber_bosch.js"),
        import("./index.js"),
    ]);
    await init();
    
    hook_panic_handler();
    setup(WebChart);
    test_method(); // check bindings of JS outputs into Rust
    main();
}