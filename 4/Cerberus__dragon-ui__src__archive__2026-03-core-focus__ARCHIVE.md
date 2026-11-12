# Core Focus Archive

This archive holds frontend routes, components, and tests that were removed from the active Dragon-R1 operator surface during the March 2026 cleanup pass.

Archived here:

- Non-core routes:
  - `app/pricing/page.tsx`
  - `app/mobile-access/page.tsx`
  - `app/test-route/page.tsx`
  - `app/api-docs/page.tsx`
  - `app/legal/terms/page.tsx`
- Legacy parallel chat stack:
  - `app/chat/enhanced-chat.tsx`
  - `components/chat-interface.tsx`
  - `components/SessionSidebar.tsx`
  - `components/StatisticsCard.tsx`
- Unused legacy UI components:
  - `components/smart-data-display.tsx`
  - `components/task-surface.tsx`
  - `components/stats-card.tsx`
  - `components/mode-switcher.tsx`
  - `components/distro-selector.tsx`
  - `components/search-bar.tsx`
  - `components/progress-bar.tsx`
  - `components/template-selector.tsx`
  - `components/navbar.tsx`
- Tests tied to archived legacy UI:
  - `components/__tests__/layout.test.tsx`
  - `components/__tests__/navbar.test.tsx`

Reason:

These files either duplicated the active chat/task workflow, served marketing/demo-only pages, or represented older UI patterns that no longer support the focused ethical web pentesting operator workflow.
