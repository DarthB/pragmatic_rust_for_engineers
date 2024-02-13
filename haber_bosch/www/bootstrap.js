init();

async function init() {
    if (typeof process == "object") {
        // We run in the npm/webpack environment.
        const [{WebChart, WebInput, WebModelRange, HaberBoschBedSetup, hook_panic_handler}, {main, setup}] = await Promise.all([
            import("haber_bosch.js"),
            import("./index.js"),
        ]);
        hook_panic_handler();
        setup(WebChart, WebInput, WebModelRange, HaberBoschBedSetup);
        main();
    } else {
        const [{WebChart, WebInput, WebModelRange, HaberBoschBedSetup, hook_panic_handler, default: init}, {main, setup}] = await Promise.all([
            import("../pkg/haber_bosch.js"),
            import("./index.js"),
        ]);
        await init();
        hook_panic_handler()
        setup(WebChart, WebInput, WebModelRange, HaberBoschBedSetup);
        main();
    }
}