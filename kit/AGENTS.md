# OrbitSmith AGENTS

This is the canonical behavior file for OrbitSmith kit.

## Mission
Provide a unified, production-ready Antigravity skills toolkit with clear guardrails.

## Source policy
- Runtime scaffold comes from `antigravity-kit` lineage.
- Skill library and operational docs come from curated `awesome-skills` lineage.
- Canonical distribution path: `kit/`

## Skill loading order
1. Exact-name match in `kit/skills/*`
2. Domain match by prefix/tag in SKILL.md
3. If multiple matches: choose the most specific skill

## Guardrails
- Prefer deterministic, auditable workflows.
- Security and quality docs in `kit/docs/SECURITY_GUARDRAILS.md` and `kit/docs/QUALITY_BAR.md` are baseline.
- Never execute destructive actions without explicit user confirmation.

## Commands (starter)
See `kit/meta/commands.json`.
