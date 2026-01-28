<template>
  <div class="normal-page">
    <div class="normal-page__sidebar">
      <aside class="universal-card">
        <h1>社区管理员</h1>
        <NavStack>
          <NavStackItem link="/moderation" label="信息统计">
            <ModrinthIcon aria-hidden="true" />
          </NavStackItem>
          <NavStackItem link="/moderation/review" label="审核资源">
            <ModerationIcon aria-hidden="true" />
          </NavStackItem>
          <NavStackItem link="/moderation/reports" label="举报">
            <ReportIcon aria-hidden="true" />
          </NavStackItem>
          <NavStackItem link="/moderation/translations" label="翻译审核">
            <LanguagesIcon aria-hidden="true" />
          </NavStackItem>
          <NavStackItem link="/moderation/appeals" label="封禁申诉">
            <ShieldIcon aria-hidden="true" />
          </NavStackItem>
          <NavStackItem link="/moderation/translation-tracking" label="汉化监控">
            <CalendarClockIcon aria-hidden="true" />
          </NavStackItem>
          <NavStackItem
            v-if="auth?.user?.role === 'admin'"
            link="/moderation/creators"
            label="高级创作者"
          >
            <StarIcon aria-hidden="true" />
          </NavStackItem>
        </NavStack>
      </aside>
    </div>
    <div class="normal-page__content">
      <NuxtPage />
    </div>
  </div>
</template>
<script setup>
import NavStack from "~/components/ui/NavStack.vue";
import NavStackItem from "~/components/ui/NavStackItem.vue";

import ModrinthIcon from "~/assets/images/utils/users.svg?component";
import ModerationIcon from "~/assets/images/sidebar/admin.svg?component";
import ReportIcon from "~/assets/images/utils/report.svg?component";
import LanguagesIcon from "~/assets/images/utils/languages.svg?component";
import ShieldIcon from "~/assets/images/utils/shield.svg?component";
import CalendarClockIcon from "~/assets/images/utils/calendar-clock.svg?component";
import StarIcon from "~/assets/images/utils/star.svg?component";

const auth = await useAuth();

definePageMeta({
  middleware: "auth",
});
</script>
