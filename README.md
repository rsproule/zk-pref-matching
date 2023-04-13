# Privacy Preserving Stable Marriage Matching

The Stable Marriage problem is a classical matching problem in which the objective is to find a stable matching between two equally-sized sets of elements, typically referred to as "men" and "women." Each element in both sets has a preference list ranking the elements from the opposite set in order of preference.

A matching is a one-to-one assignment of elements from one set to the other set, such that every element is matched with exactly one element from the opposite set. A matching is considered stable if there are no two elements, one from each set, that would prefer to be matched with each other over their current match.

The goal of the Stable Marriage problem is to find a stable matching given the preference lists of both sets of elements. The Gale-Shapley algorithm, also known as the Deferred Acceptance algorithm, is a popular algorithm that solves the Stable Marriage problem and guarantees to find a stable matching in polynomial time.

## TODO

So its fairly trivial to run the matching algorithm inside of risc zero. this is done and it works, using `65487` cycles with n=3. The issue is how do we get the preference arrays into risc zero without revealing them publicly or to a trusted 3rd party. I think this might not actually work cuz intuitively, there is shared public state in order to produce a proof. If there was some way to pass the hashes only for all of the preference arrays and verify that the computation was done on the correct data. This totally works for the example where there is a centralized trusted entity. The main problem with central entity is that they can leak data to other participants, or could be a player themselves.

## Workarounds for central matcher entity

1 version of this that is better than the *full public* version, is one where only 1 entity has access to all the preference arrays. basically can do basic encryption to guarantee that only the game master has this data.

Central entity then cannot:

1. modify anyone's preferences - enforced by zk (assert a hash)
2. incorrectly calculate the matchings - enforced by zk
3. leak some preferences to others - game theoretically enforced via some slash condition
    1. rewards users for reporting collusion
    2. could be enforced in commit/reveal

Other options for preventing collusion:

- could make all the participants submit their preferences in a commit reveal
  - need to guarantee that only the zk proof server can do the reveal

How do you prevent the central entity from posting the preferences after the reveal? thus defeating the purpose of the zk.

## Architecture

On-chain: users commit to some preference hash. This is going to be input to the proof to verify that the preferences used are the ones that users actually committed to.

Preference collector: server that accepts preference arrays + commitment hashes. generates a proof that they correctly ran the matching algo and used the correct inputs. Eventually, they send the matches back to the chain.

## Application

Whats a good use case for this?

- dating apps
- assigning partners for class project
- marriage assignment
- assigning nfts to people based on preference

## Extensions

Only reveal the actual match to the matchers. So encrypt the results with the participants public keys.

