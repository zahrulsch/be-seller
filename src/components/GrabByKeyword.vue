<script setup lang="ts">
import {
  Modal,
  Textarea,
  InputNumber,
  Input,
  Space,
  Select,
  Alert,
  SelectOption,
  message,
} from "ant-design-vue";
import { useGetConfig } from "../utils/useGetConfig";
import { CrawlByKeyword, crawlByKeyword } from "../utils/crawlByKeywordSender";
import { ref } from "vue";

interface Prop {
  show?: boolean;
}

interface Emit {
  (e: "update:show", v: boolean): void;
}

const props = defineProps<Prop>();
const emits = defineEmits<Emit>();
const payload = ref<Partial<CrawlByKeyword>>({
  config_id: undefined,
  keywords: [],
  limit_product: 100,
  name: "",
  thread_size: 1,
});

const { invoker } = crawlByKeyword({
  onSuccess(msg) {
    message.info(msg[0].toUpperCase() + msg.slice(1));
  },
  onError(e) {
    message.error(JSON.stringify(e, null, 2), 3);
  },
});
const { config } = useGetConfig();
</script>

<template>
  <Modal
    centered
    title="Kumpulkan produk berdasarkan kata kunci"
    :visible="props.show"
    @update:visible="(v) => emits('update:show', v)"
    :mask-style="{
      backgroundColor: '#e5e5e560',
    }"
    ok-text="Jalankan Tugas"
    cancel-text="Batal"
    @ok="invoker(payload)"
  >
    <Space style="width: 100%" direction="vertical">
      <Alert show-icon message="Hanya mendukung marketplace Shopee"></Alert>
      <Textarea
        :auto-size="{
          minRows: 14,
          maxRows: 14,
        }"
        placeholder="Daftar kata kunci pisahkan dengan baris baru"
        :value="payload.keywords?.join('\n')"
        @update:value="(v) => (payload.keywords = v.split('\n'))"
      ></Textarea>
      <Input
        v-model:value="payload.name"
        addon-before="Nama"
        placeholder="Keyword fashion"
      />
      <Space>
        <InputNumber
          v-model:value="payload.thread_size"
          placeholder="1"
          addon-before="Thread Count"
          :max="25"
        />
        <InputNumber
          v-model:value="payload.limit_product"
          placeholder="1"
          addon-before="Batas Produk"
        />
      </Space>
      <Select
        v-model:value="payload.config_id"
        placeholder="Filter Namespace"
        style="width: 100%"
      >
        <SelectOption v-for="cfg in config" :value="cfg.id">{{ cfg.name }}</SelectOption>
      </Select>
    </Space>
  </Modal>
</template>
