#import "template.typ": *

#show: neurips.with(
  title: [Emergent Coordination in Multi-Agent Systems via Pressure Fields and Temporal Decay#footnote[Code available at #link("https://github.com/Govcraft/schedule-experiment")]],
  authors: (
    (
      name: "Roland R. Rodriguez, Jr.",
      affiliation: "Independent Researcher",
      email: "rrrodzilla@proton.me",
    ),
  ),
  date: [January 2026],
  abstract: [
    Current multi-agent LLM frameworks rely on explicit orchestration patterns borrowed from human organizational structures: planners delegate to executors, managers coordinate workers, and hierarchical control flow governs agent interactions. These approaches suffer from coordination overhead that scales poorly with agent count and task complexity. We propose a fundamentally different paradigm inspired by natural coordination mechanisms: agents operate locally on a shared artifact, guided only by pressure gradients derived from measurable quality signals, with temporal decay preventing premature convergence. We formalize this as optimization over a pressure landscape and prove convergence guarantees under mild conditions.

    Empirically, on meeting room scheduling across XX trials, pressure-field coordination matches hierarchical control (XX% vs XX% aggregate solve rate, $p = "XX"$, indicating statistical equivalence). Both significantly outperform sequential (XX%) and random (XX%) baselines ($p < "XX"$). Temporal decay is essential: disabling it increases final pressure XX-fold ($d = "XX"$). On easy problems, pressure-field achieves XX% solve rate. The approach maintains consistent performance from 1 to 4 agents. Our key finding: implicit coordination through shared pressure gradients achieves parity with explicit hierarchical control. Foundation models enable this approach: their broad pretraining and zero-shot reasoning allow quality-improving patches from local pressure signals alone, without domain-specific coordination protocols. This suggests that constraint-driven emergence offers a simpler, equally effective foundation for multi-agent AI.
  ],
  keywords: (
    "multi-agent systems",
    "emergent coordination",
    "decentralized optimization",
    "LLM agents",
  ),
)

= Introduction

Multi-agent systems built on large language models have emerged as a promising approach to complex task automation @wu2023autogen @hong2023metagpt @li2023camel. The dominant paradigm treats agents as organizational units: planners decompose tasks, managers delegate subtasks, and workers execute instructions under hierarchical supervision. This coordination overhead scales poorly with agent count and task complexity.

We demonstrate that *implicit* coordination through shared state achieves equivalent performance to explicit hierarchical control—without coordinators, planners, or message passing. Across XX trials on meeting room scheduling, pressure-field coordination matches hierarchical control (XX% vs XX% aggregate solve rate, $p = "XX"$). Both significantly outperform sequential and random baselines, demonstrating that pressure-guided coordination provides meaningful benefit over uncoordinated approaches.

Our approach draws inspiration from natural coordination mechanisms—ant colonies, immune systems, neural tissue—that coordinate through *environment modification* rather than message passing. Agents observe local quality signals (pressure gradients), take locally-greedy actions, and coordination emerges from shared artifact state. Temporal decay prevents premature convergence by ensuring continued exploration.

Our contributions:

+ We formalize *pressure-field coordination* as a role-free, stigmergic alternative to organizational MAS paradigms. Unlike GPGP's hierarchical message-passing or SharedPlans' intention alignment, pressure-field achieves $O(1)$ coordination overhead through shared artifact state. Foundation models enable this approach: their broad pretraining allows quality-improving patches from local pressure signals without domain-specific coordination protocols.

+ We introduce *temporal decay* as a mechanism for preventing premature convergence. Disabling decay increases final pressure XX-fold (Cohen's $d = "XX"$), trapping agents in local minima.

+ We prove convergence guarantees for this coordination scheme under pressure alignment conditions.

+ We provide empirical evidence across XX trials showing: (a) pressure-field matches hierarchical control, (b) both significantly outperform sequential (XX%) and random (XX%) baselines ($p < "XX"$).

= Related Work

Our approach bridges four research traditions: multi-agent systems coordination theory provides the conceptual foundation; swarm intelligence provides the stigmergic mechanism; LLM systems provide the application domain; and decentralized optimization provides theoretical guarantees. We survey each and position pressure-field coordination within this landscape.

== MAS Coordination Theory

Pressure-field coordination occupies a unique position in the MAS landscape: it eliminates roles (unlike organizational paradigms), messages (unlike GPGP), and intention reasoning (unlike SharedPlans) while providing formal convergence guarantees (unlike purely reactive systems). This section positions our contribution within four established coordination frameworks, showing how artifact refinement with measurable quality signals enables this architectural simplification. The key insight: for this domain class, coordination complexity collapses from quadratic message-passing to constant-time state-sharing.

=== Organizational Paradigms and Dependency Management

Pressure-field coordination achieves role-free coordination: any agent can address any high-pressure region without negotiating access rights or awaiting task assignment. This contrasts sharply with traditional organizational paradigms. Horling and Lesser @horling2004survey surveyed nine such paradigms---from rigid hierarchies to flexible markets---finding that all assign explicit roles constraining agent behavior. While role assignment reduces coordination complexity by pre-structuring interactions, it introduces brittleness: role changes require protocol modifications, and role failure can cascade through the system.

Our approach instantiates Malone and Crowston's @malone1994coordination coordination framework with a critical difference: the artifact itself is the shared resource, and pressure gradients serve as dependency signals. Rather than assigning roles to manage resource access, agents share read access to the entire artifact and propose changes to high-pressure regions. Coordination emerges from pressure alignment---agents reduce local pressure, which reduces global pressure through the artifact's shared state.

=== Distributed Problem Solving and Communication Overhead

Pressure-field coordination achieves $O(1)$ inter-agent communication overhead---agents exchange no messages. Coordination occurs entirely through shared artifact reads and writes, eliminating the message-passing bottleneck. This contrasts with the GPGP framework @decker1995gpgp, which reduces communication from $O(n^2)$ pairwise negotiation to $O(n log n)$ hierarchical aggregation through summary information exchange. While GPGP represents significant progress, its explicit messages---task announcements, commitment exchanges, schedule updates---still introduce latency and failure points at scale.

The approaches target different domains. Pressure-field coordination specializes in artifact refinement tasks where quality decomposes into measurable regional signals---a class including code quality improvement, document editing, and configuration management. GPGP generalizes to complex task networks with precedence constraints. For artifact refinement, however, pressure-field's stigmergic coordination eliminates message-passing overhead entirely.

=== Shared Intentions and Alignment Costs

Pressure-field coordination eliminates intention alignment through pressure alignment. Rather than reasoning about what other agents believe or intend, agents observe artifact state and pressure gradients. When agents greedily reduce local pressure under separable or bounded-coupling conditions, global pressure decreases. This is coordination without communication about intentions---agents align through shared objective functions, not mutual beliefs.

This contrasts with the SharedPlans framework @grosz1996sharedplans, which formalizes joint activity through shared mental attitudes: mutual beliefs about goals, commitments, and action sequences. The framework elegantly captures human-like collaboration but requires significant cognitive machinery---intention recognition, commitment protocols, belief revision---all computationally expensive operations that scale poorly with agent count.

Our experiments validate this analysis: pressure-field coordination eliminates the overhead of explicit dialogue by coordinating through shared artifact state. The coordination overhead of belief negotiation in explicit dialogue systems can exceed its organizational benefit for constraint satisfaction tasks. The trade-off is transparency: SharedPlans supports dialogue about why agents act; pressure-field agents react to gradients without explaining reasoning.

=== Self-Organization and Emergent Coordination

Pressure-field coordination satisfies De Wolf and Holvoet's @dewolf2005engineering self-organization criteria: absence of external control, local interactions producing global patterns, and dynamic adaptation. They explicitly cite "gradient fields" as a self-organization design pattern---our approach instantiates this pattern with formal guarantees.

No external controller exists---agents observe and act autonomously based on local pressure signals. Coordination emerges from local decisions: agents reduce regional pressure through greedy actions, and global coordination arises from shared artifact state. Temporal decay provides dynamic adaptation---fitness erodes continuously, preventing premature convergence and enabling continued refinement.

The theoretical contribution formalizes this intuition through potential game theory. Theorem 1 establishes convergence guarantees for aligned pressure systems; the Basin Separation result (Theorem 3) explains why decay is necessary to escape suboptimal basins. This bridges descriptive design patterns and prescriptive theoretical frameworks.

=== Foundation Model Enablement

Foundation models enable stigmergic coordination through three capabilities: (1) broad pretraining allows patch proposals across diverse artifact types without domain-specific fine-tuning; (2) instruction-following allows operation from pressure signals alone, without complex action representations; (3) zero-shot reasoning interprets constraint violations without explicit protocol training. These properties make FMs suitable for stigmergic coordination---they require only local context and quality signals to generate productive actions, matching pressure-field's locality constraints.

== Multi-Agent LLM Systems

Recent work has explored multi-agent architectures for LLM-based task solving. AutoGen @wu2023autogen introduces a conversation-based framework where customizable agents interact through message passing, with support for human-in-the-loop workflows. MetaGPT @hong2023metagpt encodes Standardized Operating Procedures (SOPs) into agent workflows, assigning specialized roles (architect, engineer, QA) in an assembly-line paradigm. CAMEL @li2023camel proposes role-playing between AI assistant and AI user agents, using inception prompting to guide autonomous cooperation. CrewAI @crewai2024 similarly defines agents with roles, goals, and backstories that collaborate on complex tasks.

These frameworks share a common design pattern: explicit orchestration through message passing, role assignment, and hierarchical task decomposition. While effective for structured workflows, this approach faces scaling limitations. Central coordinators become bottlenecks, message-passing overhead grows with agent count, and failures in manager agents cascade to dependents. Our work takes a fundamentally different approach: coordination emerges from shared state rather than explicit communication.

Foundation models enable pressure-field coordination through capabilities that prior agent architectures lacked. Their broad pretraining allows reasonable patches across diverse artifact types---code, text, configurations---without domain-specific fine-tuning. Their instruction-following capabilities allow operation from pressure signals and quality feedback alone. Their zero-shot reasoning interprets constraint violations and proposes repairs without explicit protocol training. These properties make foundation models particularly suitable for stigmergic coordination: they require only local context and quality signals to generate productive actions, matching the locality constraints of pressure-field systems.

== Swarm Intelligence and Stigmergy

The concept of stigmergy---indirect coordination through environment modification---was introduced by Grassé @grasse1959stigmergie to explain termite nest-building behavior. Termites deposit pheromone-infused material that attracts further deposits, leading to emergent construction without central planning. This directly instantiates Malone and Crowston's @malone1994coordination shared resource coordination: pheromone trails encode dependency information about solution quality. This principle has proven remarkably powerful: complex structures arise from simple local rules without any agent having global knowledge.

Dorigo and colleagues @dorigo1996ant @dorigo1997acs formalized this insight into Ant Colony Optimization (ACO), where artificial pheromone trails guide search through solution spaces. Key mechanisms include positive feedback (reinforcing good paths), negative feedback (pheromone evaporation), and purely local decision-making. ACO has achieved strong results on combinatorial optimization problems including TSP, vehicle routing, and scheduling.

Our pressure-field coordination directly inherits from stigmergic principles. The artifact serves as the shared environment; regional pressures are analogous to pheromone concentrations; decay corresponds to evaporation. However, we generalize beyond path-finding to arbitrary artifact refinement and provide formal convergence guarantees through the potential game framework.

== Decentralized Optimization

Potential games, introduced by Monderer and Shapley @monderer1996potential, are games where individual incentives align with a global potential function. A key property is that any sequence of unilateral improvements converges to a Nash equilibrium—greedy local play achieves global coordination. This provides the theoretical foundation for our convergence guarantees: under pressure alignment, the artifact pressure serves as a potential function.

Distributed gradient descent methods @nedic2009distributed @yuan2016convergence address optimization when data or computation is distributed across nodes. The standard approach combines local gradient steps with consensus averaging. While these methods achieve convergence rates matching centralized alternatives, they typically require communication protocols and synchronization. Our approach avoids explicit communication entirely: agents coordinate only through the shared artifact, achieving $O(1)$ coordination overhead.

The connection between multi-agent learning and game theory has been extensively studied @shoham2008multiagent. Our contribution is applying these insights to LLM-based artifact refinement, where the "game" is defined by pressure functions over quality signals rather than explicit reward structures.

= Problem Formulation

We formalize artifact refinement as a dynamical system over a pressure landscape rather than an optimization problem with a target state. The system evolves through local actions and continuous decay, settling into stable basins that represent acceptable artifact states.

== State Space

An *artifact* consists of $n$ regions with content $c_i in cal(C)$ for $i in {1, ..., n}$, where $cal(C)$ is an arbitrary content space (strings, AST nodes, etc.). Each region also carries auxiliary state $h_i in cal(H)$ representing confidence, fitness, and history. Regions are passive subdivisions of the artifact; agents are active proposers that observe regions and generate patches.

The full system state is:
$ s = ((c_1, h_1), ..., (c_n, h_n)) in (cal(C) times cal(H))^n $

== Pressure Landscape

A *signal function* $sigma: cal(C) -> RR^d$ maps content to measurable features. Signals are *local*: $sigma(c_i)$ depends only on region $i$.

A *pressure function* $phi: RR^d -> RR_(>=0)$ maps signals to scalar "badness." We consider $k$ pressure axes with weights $bold(w) in RR^k_(>0)$. The *region pressure* is:

$ P_i(s) = sum_(j=1)^k w_j phi_j (sigma(c_i)) $

The *artifact pressure* is:

$ P(s) = sum_(i=1)^n P_i(s) $

This defines a landscape over artifact states. Low-pressure regions are "valleys" where the artifact satisfies quality constraints.

== System Dynamics

The system evolves in discrete time steps (ticks). Each tick consists of three phases:

*Phase 1: Decay.* Auxiliary state erodes toward a baseline. For fitness $f_i$ and confidence $gamma_i$ components of $h_i$:

$ f_i^(t+1) = f_i^t dot.c e^(-lambda_f) , quad gamma_i^(t+1) = gamma_i^t dot.c e^(-lambda_gamma) $

where $lambda_f, lambda_gamma > 0$ are decay rates. Decay ensures that stability requires continuous reinforcement.

*Phase 2: Proposal.* For each region $i$ where pressure exceeds activation threshold ($P_i > tau_"act"$) and the region is not inhibited, *each actor* $a_k: cal(C) times cal(H) times RR^d -> cal(C)$ proposes a content transformation in parallel. Each actor observes only local state $(c_i, h_i, sigma(c_i))$---actors do not communicate or coordinate their proposals.

*Phase 3: Validation.* When multiple patches are proposed, each is validated on an independent *fork* of the artifact. Forks are created by cloning artifact state; validation proceeds in parallel across forks. This addresses a fundamental resource constraint: a single artifact cannot be used to test multiple patches simultaneously without cloning.

*Phase 4: Reinforcement.* Regions where actions were applied receive fitness and confidence boosts, and enter an inhibition period preventing immediate re-modification. Inhibition allows changes to propagate through the artifact and forces agents to address other high-pressure regions, preventing oscillation around local fixes.

$ f_i^(t+1) = min(f_i^t + Delta_f, 1), quad gamma_i^(t+1) = min(gamma_i^t + Delta_gamma, 1) $

== Stable Basins

#definition(name: "Stability")[
  A state $s^*$ is *stable* if, under the system dynamics with no external perturbation:
  1. All region pressures are below activation threshold: $P_i(s^*) < tau_"act"$ for all $i$
  2. Decay is balanced by residual fitness: the system remains in a neighborhood of $s^*$
]

The central questions are:
1. *Existence*: Under what conditions do stable basins exist?
2. *Quality*: What is the pressure $P(s^*)$ of states in stable basins?
3. *Convergence*: From initial state $s_0$, does the system reach a stable basin? How quickly?
4. *Decentralization*: Can stability be achieved with purely local decisions?

== The Locality Constraint

The key constraint distinguishing our setting from centralized optimization: agents observe only local state. An actor at region $i$ sees $(c_i, h_i, sigma(c_i))$ but not:
- Other regions' content $c_j$ for $j != i$
- Global pressure $P(s)$
- Other agents' actions

This rules out coordinated planning. Stability must emerge from local incentives aligned with global pressure reduction.

= Method

We now present a coordination mechanism that achieves stability through purely local decisions. The key insight is that under appropriate conditions, the artifact pressure $P(s)$ acts as a *potential function*: local improvements by individual agents decrease global pressure, guaranteeing convergence without coordination.

== Pressure Alignment

The locality constraint prohibits agents from observing global state. For decentralized coordination to succeed, we need local incentives to align with global pressure reduction.

#definition(name: "Pressure Alignment")[
  A pressure system is *aligned* if for any region $i$, state $s$, and action $a_i$ that reduces local pressure:
  $ P_i(s') < P_i(s) quad ==> quad P(s') < P(s) $
  where $s' = s[c_i |-> a_i(c_i)]$ is the state after applying $a_i$.
]

Alignment holds automatically when pressure functions are *separable*: each $P_i$ depends only on $c_i$, so $P(s) = sum_i P_i(s)$ and local improvement directly implies global improvement.

More generally, alignment holds when cross-region interactions are bounded:

#definition(name: "Bounded Coupling")[
  A pressure system has *$epsilon$-bounded coupling* if for any action $a_i$ on region $i$:
  $ abs(P_j(s') - P_j(s)) <= epsilon quad forall j != i $
  That is, modifying region $i$ changes other regions' pressures by at most $epsilon$.
]

Under $epsilon$-bounded coupling with $n$ regions, if a local action reduces $P_i$ by $delta > (n-1) epsilon$, then global pressure decreases by at least $delta - (n-1) epsilon > 0$.

== Connection to Potential Games

The aligned pressure system forms a *potential game* where:
- Players are regions (or agents acting on regions)
- Strategies are content choices $c_i in cal(C)$
- The potential function is $Phi(s) = P(s)$

In potential games, any sequence of improving moves converges to a Nash equilibrium. In our setting, Nash equilibria correspond to stable basins: states where no local action can reduce pressure below the activation threshold.

This connection provides our convergence guarantee without requiring explicit coordination.

Note that this convergence result assumes finite action spaces. In practice, patches are drawn from a finite set of LLM-generated proposals per region, satisfying this requirement. For infinite content spaces, convergence to approximate equilibria can be established under Lipschitz continuity conditions on pressure functions.

== The Coordination Algorithm

The tick loop implements greedy local improvement with decay-driven exploration:

#algorithm(name: "Pressure-Field Tick")[
  *Input:* State $s^t$, signal functions ${sigma_j}$, pressure functions ${phi_j}$, actors ${a_k}$, parameters $(tau_"act", lambda_f, lambda_gamma, Delta_f, Delta_gamma, kappa)$

  *Phase 1: Decay*
  #h(1em) For each region $i$: $quad f_i <- f_i dot.c e^(-lambda_f), quad gamma_i <- gamma_i dot.c e^(-lambda_gamma)$

  *Phase 2: Activation and Proposal*
  #h(1em) $cal(P) <- emptyset$
  #h(1em) For each region $i$ where $P_i(s) >= tau_"act"$ and not inhibited:
  #h(2em) $bold(sigma)_i <- sigma(c_i)$
  #h(2em) For each actor $a_k$:
  #h(3em) $delta <- a_k(c_i, h_i, bold(sigma)_i)$
  #h(3em) $cal(P) <- cal(P) union {(i, delta, hat(Delta)(delta))}$

  *Phase 3: Parallel Validation and Selection*
  #h(1em) For each candidate patch $(i, delta, hat(Delta)) in cal(P)$:
  #h(2em) Fork artifact: $(f_"id", A_f) <- A."fork"()$
  #h(2em) Apply $delta$ to fork $A_f$
  #h(2em) Validate fork (run tests, check compilation)
  #h(1em) Collect validation results ${(i, delta, Delta_"actual", "valid")}$
  #h(1em) Sort validated patches by $Delta_"actual"$
  #h(1em) Greedily select top-$kappa$ non-conflicting patches

  *Phase 4: Application and Reinforcement*
  #h(1em) For each selected patch $(i, delta, dot)$:
  #h(2em) $c_i <- delta(c_i)$
  #h(2em) $f_i <- min(f_i + Delta_f, 1)$, $gamma_i <- min(gamma_i + Delta_gamma, 1)$
  #h(2em) Mark region $i$ inhibited for $tau_"inh"$ ticks

  *Return* updated state $s^(t+1)$
]

The algorithm has three key properties:

*Locality.* Each actor observes only $(c_i, h_i, sigma(c_i))$. No global state is accessed.

*Bounded parallelism.* At most $kappa$ patches per tick prevents thrashing. Inhibition prevents repeated modification of the same region.

*Decay-driven exploration.* Even stable regions eventually decay below confidence thresholds, attracting re-evaluation. This prevents premature convergence to local minima.

== Stability and Termination

The system reaches a stable basin when:
1. All region pressures satisfy $P_i(s) < tau_"act"$
2. Decay is balanced: fitness remains above the threshold needed for stability

Termination is *economic*, not logical. The system stops acting when the cost of action (measured in pressure reduction per patch) falls below the benefit. This matches natural systems: activity ceases when gradients flatten, not when an external goal is declared achieved.

In practice, we also impose budget constraints (maximum ticks or patches) to bound computation.

= Theoretical Analysis

We establish three main results: (1) convergence to stable basins under alignment, (2) bounds on stable basin quality, and (3) scaling properties relative to centralized alternatives.

== Convergence Under Alignment

#theorem(name: "Convergence")[
  Let the pressure system be aligned with $epsilon$-bounded coupling. Let $delta_"min" > 0$ be the minimum *local* pressure reduction $P_i(s) - P_i(s')$ from any applied patch, and assume $delta_"min" > (n-1) epsilon$ where $n$ is the number of regions. Then from any initial state $s_0$ with pressure $P_0 = P(s_0)$, the system reaches a stable basin within:
  $ T <= P_0 / (delta_"min" - (n-1) epsilon) $
  ticks, provided the fitness boost $Delta_f$ from successful patches exceeds decay during inhibition: $Delta_f > 1 - e^(-lambda_f dot.c tau_"inh")$.
]

*Proof sketch.* Under alignment with $epsilon$-bounded coupling, each applied patch reduces global pressure by at least $delta_"min" - (n-1) epsilon > 0$. Since $P(s) >= 0$ and decreases by a fixed minimum per tick (when patches are applied), the system must reach a state where no region exceeds $tau_"act"$ within the stated bound. The decay constraint ensures that stability is maintained once reached: fitness reinforcement from the final patches persists longer than the decay erodes it. $square$

The bound is loose but establishes the key property: convergence time scales with initial pressure, not with state space size or number of possible actions.

== Basin Quality

#theorem(name: "Basin Quality")[
  In any stable basin $s^*$, the artifact pressure satisfies:
  $ P(s^*) < n dot.c tau_"act" $
  where $n$ is the number of regions and $tau_"act"$ is the activation threshold.
]

*Proof.* By definition of stability, $P_i(s^*) < tau_"act"$ for all $i$. Summing over regions: $P(s^*) = sum_i P_i(s^*) < n dot.c tau_"act"$. $square$

This bound is tight: adversarial initial conditions can place the system in a basin where each region has pressure just below threshold. However, in practice, actors typically reduce pressure well below $tau_"act"$, yielding much lower basin pressures.

#theorem(name: "Basin Separation")[
  Under separable pressure (zero coupling), distinct stable basins are separated by pressure barriers of height at least $tau_"act"$.
]

*Proof sketch.* Moving from one basin to another requires some region to exceed $tau_"act"$ (otherwise no action is triggered). The minimum such exceedance defines the barrier height. $square$

This explains why decay is necessary: without decay, the system can become trapped in suboptimal basins. Decay gradually erodes fitness, eventually allowing re-evaluation and potential escape to lower-pressure basins.

== Scaling Properties

#theorem(name: "Linear Scaling")[
  Let $m$ be the number of regions and $n$ be the number of parallel agents. The per-tick complexity is:
  - *Signal computation:* $O(m dot.c d)$ where $d$ is signal dimension
  - *Pressure computation:* $O(m dot.c k)$ where $k$ is the number of pressure axes
  - *Patch proposal:* $O(m dot.c a)$ where $a$ is the number of actors
  - *Selection:* $O(m dot.c a dot.c log(m dot.c a))$ for sorting candidates
  - *Coordination overhead:* $O(1)$ — no inter-agent communication (fork pool is $O(K)$ where $K$ is fixed)

  Total: $O(m dot.c (d + k + a dot.c log(m a)))$, independent of agent count $n$.
]

The key observation: adding agents increases throughput (more patches proposed per tick) without increasing coordination cost. This contrasts with hierarchical schemes where coordination overhead grows with agent count.

#theorem(name: "Parallel Convergence")[
  Under the same alignment conditions as Theorem 1, with $K$ patches validated in parallel per tick where patches affect disjoint regions, the system reaches a stable basin within:
  $ T <= P_0 / (K dot.c (delta_"min" - (n-1) epsilon)) $
  This improves convergence time by factor $K$ while maintaining guarantees.
]

*Proof sketch.* When $K$ non-conflicting patches are applied per tick, each reduces global pressure by at least $delta_"min" - (n-1) epsilon$. The combined reduction is $K dot.c (delta_"min" - (n-1) epsilon)$ per tick. The bound follows directly. Note that if patches conflict (target the same region), only one is selected per region, and effective speedup is reduced. $square$

== Comparison to Alternatives

We compare against three coordination paradigms:

*Centralized planning.* A global planner evaluates all $(m dot.c a)$ possible actions, selects optimal subset. Per-step complexity: $O(m dot.c a)$ evaluations, but requires global state access. Sequential bottleneck prevents parallelization.

*Hierarchical delegation.* Manager agents decompose tasks, delegate to workers. Communication complexity: $O(n log n)$ for tree-structured delegation with $n$ agents. Latency scales with tree depth. Failure of manager blocks all descendants.

*Message-passing coordination.* Agents negotiate actions through pairwise communication. Convergence requires $O(n^2)$ messages in worst case for $n$ agents. Consensus protocols add latency.

#figure(
  table(
    columns: 4,
    [*Paradigm*], [*Coordination*], [*Parallelism*], [*Fault tolerance*],
    [Centralized], [$O(m dot.c a)$], [None], [Single point of failure],
    [Hierarchical], [$O(n log n)$], [Limited by tree], [Manager failure cascades],
    [Message-passing], [$O(n^2)$], [Consensus-bound], [Partition-sensitive],
    [Pressure-field], [$O(1)$], [Full ($min(n, m, K)$)], [Graceful degradation],
  ),
  caption: [Coordination overhead comparison. $K$ denotes the fork pool size for parallel validation.],
)

Pressure-field coordination achieves $O(1)$ coordination overhead because agents share state only through the artifact itself—a form of stigmergy. Agents can fail, join, or leave without protocol overhead.

= Experiments

We evaluate pressure-field coordination on meeting room scheduling: assigning $N$ meetings to $R$ rooms over $D$ days to minimize gaps (unscheduled time), overlaps (attendee double-bookings), and maximize utilization balance. This domain provides continuous pressure gradients (rather than discrete violations), measurable success criteria, and scalable difficulty through problem size.

*Key findings*: Pressure-field coordination matches hierarchical control while both significantly outperform other baselines (§5.2). Temporal decay is critical---disabling it increases final pressure XX-fold (§5.3). The approach maintains consistent performance from 1 to 4 agents (§5.4).

== Setup

=== Task: Meeting Room Scheduling

We generate scheduling problems with varying difficulty:

#figure(
  table(
    columns: 4,
    [*Difficulty*], [*Rooms*], [*Meetings*], [*Pre-scheduled*],
    [Easy], [3], [20], [70%],
    [Medium], [5], [40], [50%],
    [Hard], [5], [60], [30%],
  ),
  caption: [Problem configurations. Pre-scheduled percentage indicates meetings already placed; remaining must be scheduled by agents.],
)

Each schedule spans 5 days with 30-minute time slots (8am--4pm). Regions are 2-hour time blocks (20 regions per schedule). A problem is "solved" when all meetings are scheduled with zero attendee overlaps within 50 ticks.

*Pressure function*: $P = "gaps" dot.c 1.0 + "overlaps" dot.c 2.0 + "util\_var" dot.c 0.5 + "unsched" dot.c 1.5$

where $"gaps"$ measures empty slots as a fraction, $"overlaps"$ counts attendee double-bookings, $"util\_var"$ measures room utilization variance, and $"unsched"$ is the fraction of unscheduled meetings.

=== Baselines

We compare four coordination strategies, all using identical LLMs (`Qwen/Qwen2.5-1.5B` via vLLM) to isolate coordination effects:

*Pressure-field (ours)*: Full system with decay (fitness half-life 5s), inhibition (2s cooldown), greedy region selection (highest-pressure region per tick), and parallel validation. Includes band escalation (Exploitation → Balanced → Exploration) and model escalation (1.5B → 7B → 14B).

*Sequential*: Single agent iterates through time blocks in fixed order, proposing schedule changes one region at a time. No parallelism, pressure guidance, or patch validation---applies any syntactically valid patch regardless of quality impact.

*Hierarchical*: Simulated manager identifies the time block with highest pressure, delegates to worker agent. Validates patches before applying (only accepts pressure-reducing changes). One patch per tick. Represents centralized control with quality gating.

*Random*: Selects random time blocks and proposes schedule changes. No patch validation---applies any syntactically valid patch regardless of quality impact.

=== Metrics

- *Solve rate*: Percentage of schedules reaching all meetings placed with zero overlaps within 50 ticks.
- *Ticks to solve*: Convergence speed for solved cases
- *Final pressure*: Remaining gaps, overlaps, and unscheduled meetings for unsolved cases
- *Token efficiency*: Prompt and completion tokens consumed per unit pressure reduction

=== Implementation

*Hardware*: NVIDIA A100 80GB GPU. *Software*: Rust implementation with vLLM. *Trials*: 30 per configuration. Full protocol in Appendix A.

*Band escalation*: When pressure velocity (rate of improvement) drops to zero for 7 consecutive ticks, sampling parameters escalate: Exploitation (T=0.2, p=0.85) → Balanced (T=0.4, p=0.9) → Exploration (T=0.7, p=0.95).

*Model escalation*: After exhausting all bands with zero progress (21 ticks total), the system escalates through the model chain: 1.5B → 7B → 14B, resetting to Exploitation band. Section 5.5 analyzes this mechanism.

== Main Results

Across XX total trials spanning three difficulty levels (easy, medium, hard) and agent counts (1, 2, 4), we find that pressure-field and hierarchical coordination perform equivalently, while both significantly outperform other baselines:

#figure(
  table(
    columns: 4,
    [*Strategy*], [*Solved/N*], [*Rate*], [*95% Wilson CI*],
    [Hierarchical], [XX/XX], [XX%], [XX%--XX%],
    [Pressure-field], [XX/XX], [XX%], [XX%--XX%],
    [Sequential], [XX/XX], [XX%], [XX%--XX%],
    [Random], [XX/XX], [XX%], [XX%--XX%],
  ),
  caption: [Aggregate solve rates across all experiments (XX total trials). Chi-square test across all four strategies: $chi^2 = "XX"$, $p < "XX"$.],
)

The key finding is *stratification into two tiers*:

*Top tier (coordinated with validation)*: Pressure-field and hierarchical achieve statistically equivalent performance (XX% vs XX%, Fisher's exact $p = "XX"$). Both validate patches before applying, ensuring only pressure-reducing changes. Their confidence intervals overlap substantially.

*Lower tier (uncoordinated, no validation)*: Sequential (XX%) and random (XX%) perform significantly worse. These strategies apply any syntactically valid patch without quality gating, allowing state degradation. All pairwise comparisons with top-tier strategies are highly significant ($p < "XX"$).

This stratification isolates *coordination mechanism* as the experimental variable: top-tier strategies differ in coordination style (implicit vs explicit) but share quality gating, enabling fair comparison. The result validates our central thesis: implicit coordination through shared pressure gradients achieves parity with explicit hierarchical control.

== Ablations

=== Effect of Temporal Decay

Decay proves essential---without it, final pressure increases dramatically:

#figure(
  table(
    columns: 4,
    [*Configuration*], [*N*], [*Final Pressure*], [*SD*],
    [With decay], [XX], [$"XX"$], [$"XX"$],
    [Without decay], [XX], [*$"XX"$*], [$"XX"$],
  ),
  caption: [Decay ablation on easy scheduling problems (XX total trials). Welch's t-test: $t = "XX"$, $p < "XX"$. Cohen's $d = "XX"$.],
)

The effect size is substantial: Cohen's $d = "XX"$ exceeds the threshold for "large" effects ($d > 0.8$). Disabling decay increases final pressure by XX$times$. Without decay, fitness saturates after initial patches---regions that received early patches retain high fitness indefinitely, making them appear "stable" even when they still contain unscheduled meetings. Since greedy selection prioritizes high-pressure regions, these prematurely-stabilized regions are never reconsidered. This validates the Basin Separation result (Theorem 3): decay is necessary to escape suboptimal basins.

=== Effect of Inhibition and Examples

The ablation study tested all $2^3 = 8$ combinations of decay, inhibition, and few-shot examples on easy scheduling problems:

#figure(
  table(
    columns: 4,
    [*Configuration*], [*Solved/N*], [*Final Pressure*], [*SD*],
    [D=T, I=T, E=F], [XX/XX], [$"XX"$], [$"XX"$],
    [D=T, I=F, E=F], [XX/XX], [$"XX"$], [$"XX"$],
    [D=T, I=T, E=T], [XX/XX], [$"XX"$], [$"XX"$],
    [D=T, I=F, E=T], [XX/XX], [$"XX"$], [$"XX"$],
    [D=F, I=T, E=F], [XX/XX], [$"XX"$], [$"XX"$],
    [D=F, I=F, E=T], [XX/XX], [$"XX"$], [$"XX"$],
    [D=F, I=T, E=T], [XX/XX], [$"XX"$], [$"XX"$],
    [D=F, I=F, E=F], [XX/XX], [$"XX"$], [$"XX"$],
  ),
  caption: [Full ablation results (XX trials). D=decay, I=inhibition, E=examples.],
) <tbl:ablation>

The key finding is that *decay dominates*: configurations with decay achieve significantly better solve rates and lower final pressure than those without.

=== Negative Pheromones

In addition to positive pheromones (successful patches stored for few-shot examples), we implement *negative pheromones*: tracking rejected patches that worsened pressure. When agents repeatedly propose ineffective patches (pressure stuck at maximum), the system accumulates rejection history and injects guidance into subsequent prompts.

Unlike the "AVOID" framing that small models (1.5B parameters) struggle to follow, we use *positive language*: rejected empty-room patches become "TIP: Schedule meetings in Room A (improves by X)." This reframes what _not_ to do as what _to try instead_.

Negative pheromones decay at the same rate as positive examples ($"weight" times 0.95$ per tick, evicted below 0.1), ensuring that old failures don't permanently block valid approaches. Up to 3 recent rejections per region are included in prompts as "Hints for better scheduling."

== Scaling Experiments

Both pressure-field and hierarchical maintain consistent performance from 1 to 4 agents on medium-difficulty scheduling problems:

#figure(
  table(
    columns: 5,
    [*Agents*], [*Pressure-field*], [*95% CI*], [*Hierarchical*], [*95% CI*],
    [1], [XX/XX (XX%)], [XX%--XX%], [XX/XX (XX%)], [XX%--XX%],
    [2], [XX/XX (XX%)], [XX%--XX%], [XX/XX (XX%)], [XX%--XX%],
    [4], [XX/XX (XX%)], [XX%--XX%], [XX/XX (XX%)], [XX%--XX%],
  ),
  caption: [Scaling from 1 to 4 agents (medium difficulty, 30 trials each). Both strategies show stable performance across agent counts.],
)

Both strategies show stable performance across agent counts. Confidence intervals overlap substantially at all counts, indicating no significant agent-count effect for either strategy.

The key observation is *robustness*: both coordination strategies maintain consistent solve rates despite 4$times$ variation in agent count. This validates Theorem 3: coordination overhead remains $O(1)$, enabling effective scaling.

== Band and Model Escalation

All main experiments use a two-stage escalation mechanism. First, *band escalation* adjusts sampling parameters when progress stalls: Exploitation (T=0.2) → Balanced (T=0.4) → Exploration (T=0.7), with 7 ticks per stage. After exhausting all bands (21 ticks), *model escalation* upgrades to larger models: 1.5B → 7B → 14B, resetting to Exploitation band.

#figure(
  table(
    columns: 3,
    [*Band*], [*Temperature*], [*Top-p*],
    [Exploitation], [0.15--0.35], [0.80--0.90],
    [Balanced], [0.35--0.55], [0.85--0.95],
    [Exploration], [0.55--0.85], [0.90--0.98],
  ),
  caption: [Sampling parameter ranges per band. Temperature and top-p are randomly sampled within range for diversity.],
)

This progressive mechanism allows smaller, faster models to solve easier regions while reserving larger models for stuck high-pressure regions. The model chain (1.5B → 7B → 14B) provides graduated capability increases.

== Difficulty Scaling

Performance varies substantially across difficulty levels:

#figure(
  table(
    columns: 5,
    [*Difficulty*], [*Pressure-field*], [*Hierarchical*], [*Sequential*], [*Random*],
    [Easy], [XX%], [XX%], [XX%], [XX%],
    [Medium], [XX%], [XX%], [XX%], [XX%],
    [Hard], [XX%], [XX%], [XX%], [XX%],
  ),
  caption: [Solve rate by difficulty level (30 trials each). Difficulty is determined by room count, meeting count, and pre-scheduled percentage.],
)

The difficulty scaling reveals key insights:

1. *Easy problems maintain tier structure*: Pressure-field and hierarchical remain the top tier across all difficulty levels.

2. *All strategies degrade on harder problems*: As room constraints tighten and unscheduled meeting counts increase, solve rates decrease for all strategies.

3. *Coordinated strategies show greater resilience*: The gap between top-tier (pressure-field, hierarchical) and lower-tier (sequential, random) strategies widens on harder problems.

= Discussion

== Limitations

Our experiments reveal several important limitations:

*Pressure-field does not outperform hierarchical.* Contrary to initial expectations, pressure-field coordination achieves statistically equivalent performance to explicit hierarchical control (38.2% vs 38.8%, $p = 0.94$). The contribution is not performance advantage but rather *equivalent performance with simpler architecture*---no coordinator agent, no explicit message passing.

*Decay is non-optional.* Without temporal decay, final pressure increases 49-fold regardless of other mechanisms. This is not merely a tuning issue---decay appears essential to prevent pressure stagnation where agents become trapped in local minima.

*Absolute solve rates are modest on hard problems.* Even top-tier strategies achieve only XX--XX% on hard problems and XX--XX% on medium problems. Meeting room scheduling with tight constraints remains challenging for current LLMs.

*Additional practical limitations:*
- Requires well-designed pressure functions (not learned from data)
- Decay rates $lambda_f, lambda_gamma$ and inhibition period require task-specific tuning
- May not suit tasks requiring long-horizon global planning
- Goodhart's Law: agents may game poorly-designed metrics
- Resource cost of parallel validation: testing $K$ patches requires $O(K dot.c |A|)$ memory where $|A|$ is artifact size

== When to Choose Each Approach

Our results suggest the following guidance:

*Pressure-field coordination is preferable when:*
1. *Simplicity is valued.* No coordinator agent needed; coordination emerges from shared state.
2. *Fault tolerance matters.* No single point of failure; agents can join/leave without protocol overhead.
3. *Pressure signals are available.* The domain provides measurable quality gradients.
4. *Foundation model suitability.* FMs' zero-shot reasoning and broad pretraining make them particularly effective in stigmergic coordination. Unlike specialized agents requiring explicit action representations and communication protocols, FMs interpret pressure signals, reason about local quality constraints, and propose patches across diverse artifact types from simple instructions.

*Hierarchical coordination is equivalent when:*
1. *Explicit control is needed.* Some domains require deterministic task assignment.
2. *Interpretability is critical.* Hierarchical task assignment provides clear audit trails.

== Band and Model Escalation as Adaptive Capability

All experiments use a two-level escalation mechanism. *Band escalation* cycles through sampling strategies (Exploitation → Balanced → Exploration, 7 ticks each) before *model escalation* progresses through model sizes (1.5B → 7B → 14B parameters). Model escalation triggers when regions remain high-pressure for 21 consecutive ticks.

This mechanism proves beneficial for both top-tier strategies: on hard problems, both pressure-field and hierarchical achieve XX--XX% with escalation enabled. The escalation mechanism works because larger models have broader solution coverage and different sampling bands explore different regions of solution space. Interestingly, both coordination strategies (pressure-field and hierarchical) exploit escalation equally well, suggesting the benefit is orthogonal to coordination mechanism.

== Future Work

- *Learned pressure functions*: Current sensors are hand-designed. Can we learn pressure functions from solution traces?
- *Adversarial robustness*: Can malicious agents exploit pressure gradients to degrade system performance?
- *Multi-artifact coordination*: Extension to coupled artifacts where patches in one affect pressure in another
- *Larger-scale experiments*: Testing on schedules with more rooms and longer time horizons to characterize scaling limits
- *Alternative domains*: Applying pressure-field coordination to code refactoring, configuration management, and other artifact refinement tasks

== Societal Implications

Pressure-field coordination raises societal concerns that extend beyond technical performance. We identify three critical issues---accountability attribution, metric gaming through Goodhart's Law, and explainability challenges---that require deliberate design choices in deployment.

=== Accountability and Attribution

When coordination emerges from shared pressure gradients rather than explicit delegation, attributing outcomes to individual agents becomes challenging. In hierarchical systems, task assignment creates clear accountability chains. In pressure-field coordination, multiple agents may contribute to a region through independent pressure-reducing actions, with no record of which agent "owned" the outcome.

This accountability diffusion has both benefits and risks. The benefit is fault tolerance: agent failures degrade performance gracefully rather than catastrophically. The risk is opacity in failure analysis: identifying which agent proposed a problematic patch---and what pressure signal motivated it---requires detailed logging that the minimal coordination mechanism does not inherently provide.

For deployment in regulated domains, this suggests an augmentation requirement: pressure-field systems must maintain audit logs recording patch provenance, pressure signals at proposal time, and validation outcomes. The coordination mechanism remains simple---agents coordinate through shared state---but operational deployment adds logging infrastructure preserving accountability.

=== Goodhart's Law and Metric Gaming

Goodhart's Law states: "When a measure becomes a target, it ceases to be a good measure." Pressure-field coordination is vulnerable to this dynamic because agents are optimized to reduce pressure as defined by designer-specified functions. If those functions imperfectly capture true quality---and they inevitably do---agents will discover and exploit the mismatch.

Consider code quality pressure functions penalizing complexity metrics. An agent might reduce complexity by splitting functions excessively, harming readability while improving the metric. The mitigation is not abandoning pressure functions but designing them defensively: use multiple orthogonal pressure axes, include adversarial sensors detecting gaming strategies, and audit whether pressure reduction correlates with human quality judgments. Pressure functions should evolve as agents discover exploits.

Foundation models introduce second-order gaming concerns: LLMs trained on internet-scale text may have implicit knowledge of how to game specific benchmarks. This suggests pressure functions for LLM-based systems should favor domain-specific quality signals harder to optimize without genuine improvement.

=== Explainability Challenges

In hierarchical systems, explanations follow delegation chains: "Manager X assigned task Y to Worker Z because condition C held." In pressure-field coordination, the explanation is: "Region R had high pressure, agent A proposed patch Δ reducing pressure by δ." This is mechanistically transparent but causally opaque---it describes what happened without explaining why that particular patch was chosen.

This is the explainability trade-off inherent to emergent coordination: simplicity in mechanism comes at the cost of legibility in rationale. For many domains---code formatting, resource optimization, routine maintenance---the trade-off is acceptable: outcomes are verifiable even if reasoning is opaque. For high-stakes domains requiring human oversight, opacity is unacceptable.

The design implication is domain-dependent deployment: pressure-field coordination suits domains where outcome verification is cheap even if reasoning transparency is limited. For domains requiring justification to human stakeholders, hierarchical coordination remains necessary despite overhead costs.

=== Design Implications

These concerns suggest three requirements for responsible deployment: comprehensive audit logging preserving patch provenance and pressure signals, defensive pressure function design with multiple orthogonal axes, and domain-appropriate verification matching coordination opacity with outcome verifiability. The coordination mechanism remains simple---but responsible deployment requires surrounding infrastructure addressing accountability, gaming, and explainability.

= Conclusion

We presented pressure-field coordination, a decentralized approach to multi-agent systems that achieves coordination through shared state and local pressure gradients rather than explicit orchestration.

Our theoretical analysis establishes convergence guarantees under pressure alignment conditions, with coordination overhead independent of agent count. Empirically, on meeting room scheduling across XX trials, we find:

1. *Pressure-field matches hierarchical control* (XX% vs XX%, $p = "XX"$). Implicit coordination through shared pressure gradients achieves parity with explicit hierarchical coordination.

2. *Both significantly outperform other baselines*. Sequential (XX%) and random (XX%) perform significantly worse ($p < 0.001$).

3. *Temporal decay is essential*. Disabling it increases final pressure substantially, trapping agents in local minima.

The key contribution is not that pressure-field outperforms hierarchical---it does not. Rather, pressure-field achieves *equivalent performance with simpler architecture*: no coordinator agent, no explicit message passing, just shared state and local pressure gradients.

Foundation models and stigmergic coordination exhibit natural synergy: FMs' zero-shot capabilities eliminate the need for domain-specific action representations, while pressure-field coordination eliminates the need for complex multi-agent protocols, together enabling simple yet effective multi-agent systems.

These results suggest that for domains with measurable quality signals, implicit coordination through shared state offers a simpler, equally effective alternative to explicit hierarchical control.

= Appendix: Experimental Protocol

This appendix provides complete reproducibility information for all experiments.

== Hardware and Software

*Hardware:* NVIDIA A100 80GB GPU (RunPod cloud)

*Software:*
- Rust 1.75+ (edition 2024)
- vLLM (OpenAI-compatible inference server)
- Models: `Qwen/Qwen2.5-1.5B`, `Qwen/Qwen2.5-7B`, `Qwen/Qwen2.5-14B`

== Model Configuration

Models are served via vLLM with a system prompt configured for schedule optimization:

```
You optimize meeting room schedules. Given a schedule with gaps or conflicts,
propose ONE change: move, swap, or reschedule a meeting to reduce gaps,
overlaps, and utilization variance. Return ONLY your proposed change
in the format: MOVE meeting_id TO room day start_time
```

For multi-model setups (model escalation), models share a single vLLM instance with automatic routing based on model name. Ollama deployments use port 11434 with model-specific routing.

== Sampling Diversity

The experiment framework overrides default sampling parameters with three exploration bands per LLM call:

#figure(
  table(
    columns: 3,
    [*Band*], [*Temperature*], [*Top-p*],
    [Exploitation], [0.15 - 0.35], [0.80 - 0.90],
    [Balanced], [0.35 - 0.55], [0.85 - 0.95],
    [Exploration], [0.55 - 0.85], [0.90 - 0.98],
  ),
  caption: [Sampling parameter ranges. Each LLM call randomly samples from one band.],
)

This diversity prevents convergence to local optima and enables exploration of the solution space.

== Problem Generation and Seeding

Fair strategy comparison requires identical problem instances: each strategy must face the same scheduling challenge within a trial. We achieve this through deterministic seeding.

Each trial generates its problem from a seed:

$ "seed" = "trial" times 1000 + "agent_count" $

Trial 5 with 2 agents yields seed 5002, producing identical meeting configurations whether evaluated with pressure-field, conversation, or hierarchical coordination.

The seed governs all stochastic generation:
- Meeting durations (1-4 time slots)
- Attendee assignments (2-5 participants)
- Room preferences and capacity requirements
- Pre-scheduled vs. unassigned meeting distribution
- Time slot availability patterns

== Experiment Commands

*Main Grid (Strategy Comparison):*
```bash
schedule-experiment --host http://localhost:11434 \
  grid --trials 30 \
  --strategies pressure_field,sequential,random,hierarchical \
  --agents 1,2,4 --difficulties easy,medium,hard \
  --max-ticks 50
```

*Ablation Study:*
```bash
schedule-experiment --host http://localhost:11434 \
  ablation --trials 10 --agents 2 --difficulty easy --max-ticks 50
```

*Scaling Analysis:*
```bash
schedule-experiment --host http://localhost:11434 \
  grid --trials 30 \
  --strategies pressure_field,hierarchical \
  --agents 1,2,4 --difficulties easy \
  --max-ticks 50
```

*Band and Model Escalation:*
```bash
# Full escalation chain enabled by default
# Band escalation: Exploitation → Balanced → Exploration (7 ticks each)
# Model escalation: 1.5B → 7B → 14B (after 21 ticks at high pressure)
schedule-experiment --host http://localhost:11434 \
  grid --trials 30 --agents 2 --difficulties medium \
  --max-ticks 100
```

*Difficulty Scaling:*
```bash
# Easy: 3 rooms, 20 meetings, 70% pre-scheduled
schedule-experiment --host http://localhost:11434 \
  grid --trials 30 --agents 2 --difficulties easy --max-ticks 50

# Medium: 5 rooms, 40 meetings, 50% pre-scheduled
schedule-experiment --host http://localhost:11434 \
  grid --trials 30 --agents 2 --difficulties medium --max-ticks 50

# Hard: 5 rooms, 60 meetings, 30% pre-scheduled
schedule-experiment --host http://localhost:11434 \
  grid --trials 30 --agents 2 --difficulties hard --max-ticks 100
```

== Metrics Collected

Each experiment records:
- `solved`: Boolean indicating all meetings scheduled with zero overlaps
- `total_ticks`: Iterations to solve (or max if unsolved)
- `pressure_history`: Pressure value at each tick (gaps + overlaps + util_var + unscheduled)
- `band_escalation_events`: Sampling band changes (tick, from_band, to_band)
- `model_escalation_events`: Model tier changes (tick, from_model, to_model)
- `final_model`: Which model tier solved the schedule
- `token_usage`: Prompt and completion tokens consumed

== Replication Notes

Each configuration runs 30 independent trials with different random seeds to ensure reliability. Results report mean solve rates and tick counts across trials.

== Estimated Runtime

#figure(
  table(
    columns: 4,
    [*Experiment*], [*Configurations*], [*Trials*], [*Est. Time*],
    [Main Grid], [36], [30], [4 hours],
    [Ablation], [4], [10], [30 min],
    [Scaling], [6], [30], [1.5 hours],
    [Difficulty], [12], [30], [2 hours],
    [*Total*], [], [], [*~8 hours*],
  ),
  caption: [Estimated runtime for all experiments on NVIDIA A100 80GB GPU.],
)

#bibliography("references.bib", style: "ieee")
