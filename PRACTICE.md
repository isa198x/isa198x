# 198x Practice

**Purpose**
This document describes how 198x works on a subject.

[`MANIFESTO.md`](MANIFESTO.md) says why the project exists. [`PRINCIPLES.md`](PRINCIPLES.md) settles disagreements. This document supplies the default method, which the principles may override.

Most of it applies family-wide. The sections marked **Code198x** govern learning design specifically, and do not generalise to a format crate or an emulator core.

---

# Four Activities

198x works through four activities.

They are not a pipeline.

Work enters at whichever one it belongs to, and often returns to an earlier one.

---

## Observe

Observe what people built.

Original software.

Original hardware.

Documentation, interviews, advertisements, magazines, photographs, schematics.

Preservation begins with evidence.

A correctly sourced artefact, verified and recorded, is a complete contribution in its own right.

---

## Understand

Understand how and why something works.

Investigate:

- how the system works
- why it was designed that way
- the constraints the designers faced
- the trade-offs they made
- what came next

The goal is not memorisation.

The goal is understanding good enough to explain, recreate, improve, question and build on.

---

## Experiment

Ask questions the historical record does not answer.

What if a machine had shipped the chipset it was designed around?

What if two components that never met were connected?

Experimentation explores the design space that the engineering allowed but history did not take.

It is bound by two rules.

**Label it.** A counterfactual is presented as a counterfactual, never as something that happened. See [`PRINCIPLES.md`](PRINCIPLES.md) § *Distinguish certainty*.

**Respect the constraints.** An experiment that ignores the period's real limits explores nothing; it is only wishing.

---

## Create

Build something that did not exist before.

Not a clone.

Not an imitation.

A creation informed by what the other three activities produced.

---

# Multiple Lenses

Every interesting problem can be examined through more than one lens.

A platform game is not only a programming exercise.

It is also an exercise in computer science, software engineering, computer engineering, mathematics, game design, graphics, audio, and the history of the people who solved it first.

Each lens asks a different question. *How do we make it work?* is not the same question as *what is the hardware doing?*, *why does this algorithm work?*, *is this enjoyable and fair?*, or *who solved this before us, and why that way?*

The lenses are the atlas's threads, each named by the question it pursues. They are enumerated once, in the atlas, so that this document and that one cannot drift apart into two differently-cut lists of the same thing.

No single lens is sufficient.

Real systems emerge from all of them at once, because real engineering problems do not respect academic boundaries.

---

# Problems, Not Products (Code198x)

The curriculum teaches projects.

A project gathers several problems into one context that makes them matter: movement, collision, rendering, animation, audio, opponent behaviour, interface, memory, performance, design.

Historical games are evidence, inspiration and case studies.

They are not templates to reproduce.

The learner's objective is never to recreate a famous game.

It is to understand the underlying problems well enough to design and build something original.

---

## Teaching through limitation (Code198x)

The learner meets a limitation.

The limitation raises a question.

The technique answers it.

The abstraction removes the pain.

This mirrors how the field itself developed, and it is why a technique is introduced *after* the learner has felt the need for it rather than before.

Its binding form is Code198x's deprecation-pair decision: teach the honest naive version first, then the upgrade, once the learner has hit the naive version's limit in their own running program.

---

# When to Stop Pulling

A thread is not free.

Time spent on one is time not spent on another.

A thread earns its time when the answer would change something:

- what gets built
- what gets taught
- what gets corrected
- what connects to something already here

Stop pulling when:

The next answer would change nothing downstream.

You are collecting facts rather than connecting them.

Ten sources are repeating one source.

The thread has left the project's scope, and continuing means widening the scope to fit.

Stopping is not abandoning.

Write the question down.

An unanswered question is a legitimate artefact.

It waits for evidence, or for someone else to pull on it.

---

# Representing Knowledge

198x should move gradually from documents towards a connected body of knowledge.

Pages should assemble information from shared knowledge rather than duplicate it.

The atlas is this model populated: it is where the entities, claims, evidence and relationships below live.

```
Entities

↓

Facts

↓

Evidence

↓

Relationships

↓

Narratives
```

**Entities** are the things the project talks about: computers, CPUs, chips, peripherals, operating systems, languages, people, companies, games, books, magazines, formats, tools, standards.

**Facts** belong to entities — release date, CPU, memory, manufacturer, supported formats, instruction timing, hardware revisions. Each fact should be independently maintainable.

**Evidence** attaches to facts: sources, confidence, verification status, evidence type, contradictory evidence, notes.

**Relationships** are explicit and reusable across the family — *Spectrum* `uses` *Z80A*.

**Narratives** are assembled from entities and facts. A narrative is not a fact, and facts must stay reusable independently of the documents describing them.

## Grow incrementally

Do not stop development to design a perfect ontology.

Extract shared knowledge gradually.

Let the model emerge from repeated use.
