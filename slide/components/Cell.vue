<script setup lang="ts">
import { ref } from 'vue'
const props = defineProps<{
  width: string
}>()
const theme = ref({
    color: 'red',
})
</script>

<template>
  <div class="box" style="--ratio: calc(6 / 8)"></div>
</template>

<style scoped>
.box {
  /* --- レイアウト側 --- */
  /* width: 100px; */
  width: v-bind('props.width');
  aspect-ratio: 4 / 3;
  border: 2px solid #333333;
  position: relative;

  /* --- 縦線を描くためのカスタムプロパティ --- */
  /* ここに fallback 値を書いておくと安心 */
  --ratio: 0.75;

  background: linear-gradient(
    to right,
    #88b3f7 calc(var(--ratio) * 100%),
    #f79388 calc(var(--ratio) * 100%)
  );
}

/* 疑似要素１本で縦線を引く */
.box::after {
  content: "";
  position: absolute;
  top: 0;
  bottom: 0;
  /* ──────────────────────────────
     left = 外枠幅 × n/(n+x)
     calc() / var() が使えるブラウザなら
     var(--ratio) を 0〜1 で渡せば OK
     ──────────────────────────────*/
  left: calc(var(--ratio) * 100%);

  /* 好きな線種に変更可 */
  border-left: 2px solid #333333;
  /* 線幅ぶんだけ中央合わせしたい場合
     transform: translateX(-1px);
  */
}
</style>
