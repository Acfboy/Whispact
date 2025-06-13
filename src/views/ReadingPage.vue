<template>
    <v-card :title="title" variant="text">
        <v-card-text>{{  body }}</v-card-text>
    </v-card>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router';

 
const route = useRoute();
const title = route.query.title as string;
const body = route.query.body as string;
</script>