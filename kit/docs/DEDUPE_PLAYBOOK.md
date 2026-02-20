# OrbitSmith Dedupe Playbook

## Goal
Reduce overlapping skills and keep one canonical skill per intent.

## Inputs
- `kit/meta/skills-tags-index.json`
- `kit/meta/semantic-duplicate-candidates.json`
- `kit/meta/duplicate-skillmd-groups.json`

## Decision rules
1. Exact duplicate content -> keep one canonical, archive the rest.
2. Semantic overlap -> keep the skill with:
   - clearer scope
   - better examples
   - stricter guardrails
3. Preserve aliases by adding redirect notes in removed skill folders.

## Output
- `kit/meta/dedupe-decisions.json` (to be filled manually)
