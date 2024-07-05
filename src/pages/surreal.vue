<template>
  <LayoutTile title="SurrealDB" description="Connect to SurrealDB - everything is in rust!">
    <div class="mx-auto max-w-xl lg:mr-0 lg:max-w-lg">
      <div class="grid grid-cols-1 gap-x-8 gap-y-6">
				<div>
					<label for="command-input" block text-sm text-white font-semibold leading-6>Command input</label>
					<div mt="2.5">
						<input id="command-input" v-model="input" type="text" name="command-input" block w-full border-0 rounded-md bg="white/5" px="3.5" py-2 text-white shadow-sm ring-1 ring="white/10" ring-inset sm="text-sm leading-6" focus="ring-2 ring-emerald-500 ring-inset">
					</div>
				</div>
				<div mt-8>
					<label for="command-output" block text-sm text-white font-semibold leading-6>Command Output</label>
					<div mt="2.5">
						<textarea id="command-output" v-model="result" name="command-output" rows="10" block w-full border-0 rounded-md bg="white/5" px="3.5" py-2 text-white shadow-sm ring-1 ring="white/10" ring-inset sm="text-sm leading-6" focus="ring-2 ring-emerald-500 ring-inset" />
					</div>
				</div>
        <div class="flex justify-end">
          <Btn type="submit" @click="queryDB">
            Execute Query
          </Btn>
        </div>
      </div>
    </div>
  </LayoutTile>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
const result = ref("");
const input = ref("");

// onMounted(async () => {
//   await invoke<string>('connect').then((message) => {
//     console.log(message)
//   })
// })

async function queryDB() {
  try {
    invoke<any>('execute_query', { query: input.value }).then((message) => {
      result.value = JSON.stringify(message);
    })
  }catch(e) {
    console.log('Got an error: ', e)
  }
}
</script>
