# Local Windows TLS adaptations

Upstream sources: parking_lot 0.12.5 and parking_lot_core 0.9.12 from the local Cargo registry. Original MIT/Apache-2.0 notices and licenses are retained.

The workspace patches these crates for the manual-mapped payload:

- Windows parking_lot_core thread parking data uses OS-managed FlsAlloc/FlsGetValue/FlsSetValue storage instead of compiler static thread_local storage. Values are dropped by the OS FLS callback on thread/fiber exit.
- Windows reentrant lock identities use GetCurrentThreadId rather than the address of a compiler TLS variable.
- Non-Windows paths remain upstream behavior. Lock algorithms and public lock interfaces are unchanged.

FLS keys have process lifetime. This is not a claim that arbitrary PE static TLS is supported by the mapper, nor permission to unmap a live payload with outstanding callbacks.

Regression: core::threading::tests::native_threads_contend_and_reenter_without_compiler_tls_keys forces shared-lock contention and recursive lock acquisition on six CreateThread workers.
