type NonPromise<T> = T extends Promise<infer V> ? V : T;
type AsAsync<F extends Function> = (...args: Parameters<F>) => Promise<NonPromise<ReturnType<F>>>;
type BridgedModule<T> = {
	[K in keyof T]: T[K] extends Function ? AsAsync<T[K]> : T[K]
};

type core_module = BridgedModule<typeof import("@core")>;


declare global {
	const core: core_module;
}

export default global;
