<template>
  <v-container fluid>
        <v-row justify="center">
         <b> 时光胶囊 </b>
        </v-row>
 </v-container>
   <v-container fluid>
     <v-textarea label="属于你们的打卡瞬间" counter auto-grow class="ma-6"></v-textarea>
      <v-dialog max-width="500" persistent transition="dialog-bottom-transition">
        <template v-slot:activator="{ props: activatorProps }">
          <v-row justify="center" class="ma-5">
            <v-btn 
             v-bind="activatorProps"
             rounded
             block>确定
            </v-btn>
          </v-row>
       </template>
       <template v-slot:default="{ isActive }">
           <v-card>
             <v-card-text>
               请与另一台手机碰一碰
             </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>

        <v-btn
          text="退出"
          @click="isActive.value = false"
        ></v-btn>
      </v-card-actions>
    </v-card>
       </template>
      </v-dialog>
 </v-container>
</template>

<script setup="ts">


</script>