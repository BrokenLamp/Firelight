
export const none = null;

export const some = (value: any): boolean =>
    value != null && value != undefined;

export default <T, R>(value: T) => (...branches: [any, (v: T) => R][]): R => {
    if (!branches) throw "there are no branches to match";
    const found = branches.find((branch: [any, (v: T) => R]) => {
        if (branch[0] === value) return true;
        if (branch[0] === true) return true;
        if (typeof branch[0] === "function") {
            return branch[0](value);
        }
    });
    if (!found) return branches.slice(-1)[0][1](value);
    const fun = found[1];
    return fun(value);
};
