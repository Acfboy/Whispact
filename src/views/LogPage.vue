<template>
    <v-container>
        <v-row justify="center" align="center" class="fill-height">
            <v-col cols="auto">
                <v-progress-circular v-show="log.length == 0" color="primary" indeterminate></v-progress-circular>
            </v-col>
        </v-row>
        <v-card variant="flat">
            {{ log }}
        </v-card>
    </v-container>
</template>

<script setup lang="ts">
import { BaseDirectory } from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/plugin-fs";
import { onMounted, ref } from "vue";

const log = ref("")

onMounted(async () => {
    log.value = await readTextFile("logs/log.log", { baseDir: BaseDirectory.AppData })
})
</script>