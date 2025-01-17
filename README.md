# Kernel Model Check

## Build Model

A kernel model is a combination of abstract state and methods.

$$
M = (O,S) =((Events,Scheduler),(TaskBlocks,CurrentTask,\cdots))
$$

The only way to update a kernel model is calling a Event

$$
\Pi(M,event) = M.call(event)
$$

The resulting $\Pi(M, event)$ is a set that contains all possible model states after calling $event$.

For an abstract function $f$, a real kernel $K$ with state $S_k$, and a kernel model $M$ with state $S_M$, we say $S_M$ matches $S_K$ if

$$
S_M = f(S_K)
$$

For an execution process of a user app, the part that a kernel mostly focuses on can be abstracted as a Event sequence.

$$
A = [event_1, event_2, \cdots]
$$

OS correctness can be marked as

$$
\begin{aligned}
K \sim M \iff & \forall A = [event_1,event_2,\cdots],\ S_{M,0} = f(S_{K,0}) \rightarrow  \\
&f(S_{K.call(event_1)}) \in S_{\Pi(M,event_1)} \\
&\wedge f(S_{K.call(event_1).call(event_2)}) \in S_{\Pi(\Pi(K,event_1),event_2))} \\
&\wedge \cdots \\
\end{aligned}
$$

For each event sequence, after each execution step, the abstract state of the real kernel must be an element of all possible states of the kernel model. 

## Test Routine

A normal test routine can be designed as

1. Match initial states, config kernel model such that

$$
S_M = f(S_K)
$$

2. Execute an event on both kernel and model, check if satisfies

$$
f(S_{K.call(event)}) \in S_{\Pi(M,event)}
$$

3. If yes, update model state as

$$
S_M' \leftarrow f(S_K')
$$

4. Loop until a violation occurs.

## Implementation

1. Describe Abstract Model. Design a customized kernel description language. Portable components.

   >KML (Kernel Model Language), grammar defined in `kml.g4`, example in `demo.kml`.

2. Kernel Input and Output. The target kernel runs on an emulator (e.g. QEMU) normally. How to manage input and output of the kernel?

3. How to define observable traits and abstract states? How to track states and traits in the kernel which runs on an emulator?

## Reference

* [A Practical Verification Framework for Preemptive OS Kernels](https://brightfu.github.io/research/certiucos/paper.pdf)
