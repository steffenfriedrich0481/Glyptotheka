Glyptotheka Constitution 

Core Principles

I. Maintaining the no_std Core

    - Every modules/*-core crate must always be buildable with #![no_std].
    - Dependencies on std are limited to cfg(test) or host-only crates; do not introduce #[cfg(feature = "std")] into the runtime proper.
    - Do not add runtime crates (including tokio or embassy) as dependencies of modules/*-core. Isolate tokio dependencies in *-std crates and embassy dependencies in *-embedded crates.
    - Functionality that requires std must be provided via an adapter layer in a separate crate to keep boundaries clear. Reason: Preserve guaranteed operation and deterministic behavior on embedded targets.

II. Test Integrity and Strict CI Enforcement

    - Each user story starts from a failing test; upon completion keep ./scripts/ci-check.sh all green.
    - Commenting out, #[ignore]-ing, or deleting tests is prohibited.
    - Record test results in planning/task documents; on failures document cause and remediation. Reason: Quantitatively assure quality and detect regressions early in CI.

III. Reference-Consistent Design

    - Investigate the relevant implementations in protoactor-go and Apache Pekko; translate and adopt the design intent idiomatically in Rust.
    - Document diffs against reference implementations; provide justification and follow-up for intentional deviations.
    - Reflect reference findings in spec/plan/tasks with traceable change history. Reason: Reuse mature actor runtime knowledge and maintain design consistency.

IV. Module Structure and Type Isolation

    - Adopt the 2018 module system; do not use mod.rs.
    - Define only one struct or one trait per file; separate unit tests using the target/tests.rs pattern.
    - Internally prefer fully qualified paths (FQCN) in use statements; reserve a prelude only for external public exposure.
    - Comply with docs/guides/module_wiring.md conventions and custom lints (module-wiring, type-per-file, etc.). Reason: Maintain module readability and build discipline enabling automated tooling validation.

V. Aggressive Design Evolution

    - While still pre-release, prioritize optimal design and do not fear breaking changes.
    - Define migration steps and impact scope for breaking changes in spec/plan/tasks beforehand and present them in a reviewable form.
    - Immediately update related documentation and templates after changes. Reason: Shorten feedback loops and reduce future technical debt.

VI. Inductively Consistency-Driven Design

    - Before starting new implementation or improvements, always examine existing code with equivalent responsibilities; understand adopted design patterns, abstractions, and naming at a transcription level.
    - If a predominant method (e.g., trait objects) is unified across most existing implementations, continue using it by default; document reason and impact in spec/plan/tasks when intentionally diverging.
    - Record investigation results and source paths; present inductively derived rules in review. Reason: Preserve whole-codebase consistency and minimize learning and operational cost.

VII. Lifetime-First Design

    - Favor borrowing and explicit lifetimes over ownership transfer in APIs; default to reuse of stack-resident data.
    - Restrict heap allocation via alloc to unavoidable locations; use buffer reuse or object pools to reduce allocation count and fragmentation. Log rationale and measurements for allocation sites in spec/plan/tasks.
    - Minimize copying of messages or contexts even when using containers like AnyMessage; design APIs so lifetime coherence is verifiable in code review.
    - If lifetime or ownership boundaries become complex, document safety rationale beforehand and submit for review verification.
    - Prohibit reusing the same type or trait for distinct responsibilities; define dedicated types (e.g., ActorError vs SendError) when needed to clarify boundaries. Reason: Prevent fragmentation and throughput degradation from allocation, sustaining deterministic performance on embedded systems.

Implementation Conventions and Structural Constraints

    - modules/actor-core and modules/utils-core must strictly remain #![no_std] while leveraging alloc libraries and preserving embedded settings like panic-halt.
    - Exclude runtime dependencies such as tokio or embassy from modules/*-core; restrict them to corresponding *-std / *-embedded crates.
    - Even when tests or benchmarks require std, confine usage to cfg(test) or dedicated crates.
    - Consolidate unit tests into tests.rs at the same level as the target module; separate common helpers into distinct modules.
    - Write rustdoc (///, //! comments) in English; write other comments and documentation in Japanese.
    - Prefer FQCN-based use statements; limit re-exports to the direct parent of leaf modules.
    - Custom lints under lints/ must follow rules clarified in lints/*/README.md; run makers ci-check -- dylint before and after work to detect deviations. Update and verify content before allowing AI or CI auto-fixes.
    - Direct use of alloc::sync::Arc or spin::Mutex is forbidden; shared references and locks must be implemented via modules/utils-core abstractions: Shared/ArcShared/RcShared and AsyncMutexLike/SyncMutexLike. In no_std default to SpinAsyncMutex/SpinSyncMutex; extend the same abstractions for platform-specific implementations as needed.
    - Use the suffix _shared / Shared for Shared-related types and variable names; prohibit Handle prefixes/suffixes. Minimize shared references considering embedded load; avoid Shared when alternatives exist. Also forbid suffixes that reflect implementation convenience such as Driver or Facade.
    - Runtime APIs and internal data structures adopt lifetime-centric design prioritizing borrowing; when heap allocation is necessary share purpose/frequency/reuse strategy in specifications and tasks. Do not pass data through an allocator if stack or static representation is possible.
    - Public API types/traits/methods must not include historical naming like Typed; adopt names presuming static typing such as ActorRef<M>. For Any-based internal structures, use names like Internal to clearly differentiate.
    - Prioritize designs that avoid cyclic references; do not introduce complex APIs relying on chains of Weak. When needed, dissolve dependencies via functional abstractions such as MapSystemFn and verify absence of cyclic structures in design review.
    - Considering embedded targets, major components must provide pluggable substitution points (abstractions and implementation switching). Avoid excessive use of trait objects; favor static binding and generics. When trait objects are used, analyze and record impact on performance and code size.
    - Communication/serialization/transport layers must remain replaceable through abstract interfaces, not fixed to specific implementations (e.g., protobuf, TCP). Prefer formats/transport usable in no_std; do not assume technologies unsupported in embedded environments (e.g., protobuf).

Development Flow and Review Procedure

    - For new features or breaking changes use the OpenSpec workflow to prepare plan/spec/tasks and pass constitution checks before implementation starts.
    - Use the Serena MCP tool for code investigation and editing; retain acquired knowledge as recorded artifacts.
    - Before implementation, read related modules or analogous functionality; record diffs versus dominant design patterns in plan/spec/tasks.
    - Completion criteria for all work are both ./scripts/ci-check.sh all and makers ci-check -- dylint being green; attach result logs to reviews.
    - Present major diffs alongside comparison results with protoactor-go / Apache Pekko during review.
    - For each completed user story report test results and documentation updates enabling progressive demos/deployments.

Governance

    - This constitution is the sole authoritative source of fraktor-rs development norms and supersedes other documents.
    - Revisions require approval of an OpenSpec proposal and must include reason for change, impact analysis, and a plan for updating templates.
    - Versioning follows SemVer: replacing or removing a principle increments MAJOR; adding a principle or significantly expanding operational guidance increments MINOR; wording adjustments or minor supplements increment PATCH.
    - All PRs must attach a constitution compliance checklist in review comments and include CI results plus test execution logs.
    - Maintainers audit compliance quarterly; if deviations are found they record corrective plans and deadlines.

Version: 1.0.0 | Ratified: 2025-11-16 | Last Amended: 2025-11-16
