# Problem: Playlist

CSES URL: https://cses.fi/problemset/task/1141

## Statement

Given a playlist of `n` songs, determine the length of the longest contiguous
subarray in which every song appears at most once.

See the official CSES page for complete details.

Source: https://cses.fi/problemset/task/1141

## Solution

Pattern: Sliding Window

Technique: Last Seen Index

Time: O(n)

Space: O(n)

## Insight

Maintain the length of the current suffix ending at each position that contains
only distinct values.

For each song:

- Increase the current window length by one.
- If the song has been seen before, the window cannot extend past its previous
  occurrence, so the new length is limited to the distance from that occurrence.
- Track the maximum window length seen.

Rather than storing the left boundary of the window explicitly, this solution
stores only the current window length. The recurrence

`current = min(current + 1, index - previous_index)`

is equivalent to the classic two-pointer sliding window solution while using
less explicit state.

## Edge Cases Checklist

- A playlist containing a single song.
- All songs are unique (answer is `n`).
- All songs are identical (answer is `1`).
- A repeated song immediately follows its previous occurrence.
- A repeated song occurs after the window has already moved forward.
- Multiple different repeated songs appear within the same traversal.