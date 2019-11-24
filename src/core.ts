interface CoreInterface {
    setSoundscape: (name: string) => void;
}

const core: CoreInterface = window.require("core");

export default core;
