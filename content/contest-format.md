+++
title = "Contest Format"
date = 2026-02-28
+++

ICMT consists of three rounds: the **Power Round**, the **Constellation Round**, and the **Individual Round**. The two divisions have separate rankings; scores in one division do not affect the other.

All answers must be submitted according to the [acceptable answer formats document](/answer-formats.pdf). Answers not submitted in the specified formats will be marked incorrect, even if mathematically equivalent. See also the [Common mathematical notions](/common-notions) page for notation conventions used in the contest.

## Power Round

### Round Structure

- You will work in teams to solve multi-part, proof-oriented questions. The entire round will be themed around a single topic, which will be revealed during the test.
- You have **90 minutes** to complete this round.
- Problems that use the words "compute," "classify," "find," "draw," "give an example," or "write" require only an answer; no explanation or proof is needed.
- All other questions, including those that say "show" or "prove," require proofs. Partial credit may be available for proof-based questions; partial reasoning will not receive credit for computational questions.

### Scoring

- Each question has its own point value. The difficulties of the questions are generally indicated by the point values assigned to them.

## Constellation Round

The round structure and scoring are slightly different between Division A and Division B.

### Division A

#### Round Structure

- This is a team round. There are **40 problems**, divided into 6 constellations of 6 problems each, plus 4 other problems. Your team will have **90 minutes** total.
- Each of the 6 constellations is associated with a different subject: **Algebra, Arithmetic (Number Theory), Calculus, Combinatorics, Linear Algebra, and Probability**.
- Problems are **NOT** ordered by difficulty.
- Your team may submit an answer to a problem at any time. Each team has up to **44 submissions**.
- You may resubmit if you get a problem wrong, but only the **latest** submission per problem counts.
- During the round, there will be a live scoreboard displaying teams' scores and which problems they have solved (except for Problem 4). The scoreboard will not be updated in the last 5 minutes.

#### Scoring

Your team's score is the sum of the following four categories:

**Base Points:** Each correct answer receives 15 points. Incorrect answers receive 0 points.
- One problem (Problem 4) has a different scoring system (which will be revealed during the test), and is worth up to 75 points. It is ineligible for bonus points.
- The maximum number of base points is $15 \cdot 39 + 75 = \mathbf{660}$.

**Speed Bonus:** Bonus points are awarded for solving problems before other teams.
- The $N$th team to solve a problem will receive $\max(16 - N, 0)$ bonus points. In other words, the first team to solve a problem will receive 15 bonus points, the second team will receive 14 bonus points, etc. The sixteenth team and later teams will receive no bonus points.
- We expect roughly 70 teams to be competing.
- The maximum number of speed bonus points is $15 \cdot 39 = \mathbf{585}$.

**Depth Bonus:** Bonus points are awarded for solving multiple problems in the same constellation.
- Within a constellation, $3(N - 1)$ bonus points are awarded on the $N$th correct answer, for a total bonus of $3N(N - 1)/2$ depth points per constellation.

  | Problems solved in constellation | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
  | -------------------------------- | - | - | - | - | -- | -- | -- |
  | Total Depth Bonus                | 0 | 0 | 3 | 9 | 18 | 30 | 45 |

- The maximum number of depth bonus points is $6 \times 45 = \mathbf{270}$.

**Breadth Bonus:** Bonus points are awarded for solving problems in every constellation.
- For $N > 0$, $18N + 24$ bonus points are awarded upon solving at least $N$ problems in each of the 6 constellations, for a total bonus of $9N(N + 1) + 24N$ breadth points.

  | Problems solved in every constellation | 0 | 1  | 2   | 3   | 4   | 5   | 6   |
  | -------------------------------------- | - | -- | --- | --- | --- | --- | --- |
  | Total Breadth Bonus                    | 0 | 42 | 102 | 180 | 276 | 390 | 522 |

The maximum score on the Division A Constellation Round is $660 + 585 + 270 + 522 = \mathbf{2037}$ points.

### Division B

#### Round Structure

- This is a team round. There are **34 problems**, divided into 5 constellations of 6 problems each, plus 4 other problems. Your team will have **90 minutes** total.
- Each of the 5 constellations is associated with a different subject: **Algebra, Arithmetic (Number Theory), Calculus, Combinatorics, and Probability**.
- Problems are **NOT** ordered by difficulty.
- Your team may submit an answer to a problem at any time. Each team has up to **38 submissions**.
- You may resubmit if you get a problem wrong, but only the **latest** submission per problem counts.
- During the round, there will be a live scoreboard displaying teams' scores and which problems they have solved (except for Problem 4). The scoreboard will not be updated in the last 5 minutes.

#### Scoring

Your team's score is the sum of the following four categories:

**Base Points:** Each correct answer receives 15 points. Incorrect answers receive 0 points.
- One problem (Problem 4) has a different scoring system (which will be revealed during the test), and is worth up to 75 points. It is ineligible for bonus points.
- The maximum number of base points is $15 \cdot 33 + 75 = \mathbf{570}$.

**Speed Bonus:** Bonus points are awarded for solving problems before other teams.
- The $N$th team to solve a problem will receive $\max(16 - N, 0)$ bonus points. In other words, the first team to solve a problem will receive 15 bonus points, the second team will receive 14 bonus points, etc. The sixteenth team and later teams will receive no bonus points.
- We expect roughly 70 teams to be competing.
- The maximum number of speed bonus points is $15 \cdot 33 = \mathbf{495}$.

**Depth Bonus:** Bonus points are awarded for solving multiple problems in the same constellation.
- Within a constellation, $3(N - 1)$ bonus points are awarded on the $N$th correct answer, for a total bonus of $3N(N - 1)/2$ depth points per constellation.

  | Problems solved in constellation | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
  | -------------------------------- | - | - | - | - | -- | -- | -- |
  | Total Depth Bonus                | 0 | 0 | 3 | 9 | 18 | 30 | 45 |

- The maximum number of depth bonus points is $5 \times 45 = \mathbf{225}$.

**Breadth Bonus:** Bonus points are awarded for solving problems in every constellation.
- For $N > 0$, $15N + 20$ bonus points are awarded upon solving at least $N$ problems in each of the 5 constellations, for a total bonus of $15N(N + 1)/2 + 20N$ breadth points.

  | Problems solved in every constellation | 0 | 1  | 2  | 3   | 4   | 5   | 6   |
  | -------------------------------------- | - | -- | -- | --- | --- | --- | --- |
  | Total Breadth Bonus                    | 0 | 35 | 85 | 150 | 230 | 325 | 435 |

The maximum score on the Division B Constellation Round is $570 + 495 + 225 + 435 = \mathbf{1725}$ points.

## Individual Round

### Round Structure

- This is an individual test. You will have **75 minutes** total.
- There are **12 problems** total. Two problems are estimation problems; you will get more points for an answer closer to the exact answer. The remaining 10 problems are standard short answer problems.
- Problems are roughly ordered by difficulty.

### Scoring

- The 10 standard short answer problems are worth 1 point for a correct answer, and 0 points for an incorrect answer.
- The 2 estimation problems have separate scoring systems, which will be explained in their respective problem statements. Each estimation problem will give at least 0 points and at most 1 point.
- Your total score on the round will be the sum of the points scored on all 12 problems.

### Tiebreakers

In case of a tie, the following tiebreaker system will be used:

1. First, the 10 short answer problems are ordered from hardest to easiest, based on the number of correct submissions.
   - The estimation problems will not be taken into account for the tiebreaker system.
   - In case of short answer problems having an equal number of solves, the problems will then be ordered from later in the test to earlier.
2. The $n$th hardest problem is assigned a tiebreaking value of $2^{-n}$. For example, the hardest problem will have a tiebreaking value of $\frac{1}{2}$, the second hardest problem will have a tiebreaking value of $\frac{1}{4}$, et cetera.
3. Each student's tiebreaking index is calculated as the sum of their original score and the tiebreaker values of the problems that they correctly answered.
4. Students are ordered by their tiebreaking index, determining their tiebroken rank.

Roughly speaking, among those tied for the same score, whoever solved the hardest short answer problem is placed the highest, followed by the one who solved the next hardest, and so on.

## Overall Team Score

A team's final ranking is determined entirely by their Team Score, which is the sum of the normalized scores in each round:

$$\text{Team Score} = N_{\text{Power}} + N_{\text{Constellation}} + N_{\text{Individual}}$$

### Power Round

Let the team's final Power Round score be $A$, and let the tenth place score on the Power Round (for that team's respective division) be $B$. Then:

$$N_{\text{Power}} = \frac{A}{B}$$

### Constellation Round

Let the team's final Constellation Round score be $A$, and let the top score on the Constellation Round (for that team's respective division) be $B$. Then:

$$N_{\text{Constellation}} = \frac{A}{B}$$

### Individual Round

For each of the contestants on the team, let the contestant's Individual Round score be $A$, and let the ninetieth percentile score on the Individual Round (for that contestant's respective division) be $B$. Then the contestant has a normalized score of $\frac{A}{B}$. The team's normalized score $N_{\text{Individual}}$ is the average of all of the normalized scores of the contestants on the team. (In particular, a team's normalized individual score is not punished for the team having fewer than 4 members.)
