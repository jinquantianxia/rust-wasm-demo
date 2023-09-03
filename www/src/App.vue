<template>
  <div>Hello WASM!</div>
</template>

<script setup lang="ts">
import init from './wasm/hello_wasm_bg.wasm?init';
import { onMounted } from 'vue';

interface Instance {
  exports: {
    add_range: (a: number, b: number) => number;
  }
}
onMounted(() => {
  init({}).then((instance) => {
    const jsStartTime = new Date().getTime();
    console.log('js start time: ', jsStartTime)
    const jsSum = addRange(1, 1000000000);
    const jsEndTime = new Date().getTime();
    console.log('js end time: ', jsEndTime);
    console.log('jsresult: ', jsSum);
    console.log('js calc time: ', jsEndTime - jsStartTime);
    const rsStartTime = performance.now(); // 从页面加载到执行该语句之间的时间间隔，是一个衡量值
    console.log('rs start time: ', rsStartTime);
    const rsSum = (instance as Instance).exports.add_range(1, 1000000000);
    const rsEndTime = performance.now();
    console.log('rs end time: ', rsEndTime);
    console.log('rsresult: ', rsSum);
    console.log('rs calc time: ', rsEndTime - rsStartTime);
  })

})

function addRange(a: number, b: number) {
  let sum = 0;
  for (let i = a; i < b; i++) {
    sum += i;
  }
  return sum;
}

</script>
