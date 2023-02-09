<script setup lang="ts">
import {
  Modal,
  Space,
  Input,
  Image,
  Divider,
  TypographyText,
  InputPassword,
  Spin,
  Button,
} from "ant-design-vue";
import { ref, watch } from "vue";
import { getBigSellerPreLoginData } from "../utils/getBigSellerPreLoginData";

const image = ref("");
const { invoker, pending } = getBigSellerPreLoginData({
  onError: console.log,
  onSuccess: (data) => {
    image.value = data.v_code_response.data;
  },
});

interface Prop {
  show?: boolean;
}

interface Emit {
  (e: "update:show", v: boolean): void;
}

const props = defineProps<Prop>();
const emits = defineEmits<Emit>();

watch(
  () => props.show,
  (s) => {
    if (s) {
      invoker();
    }
  }
);
</script>

<template>
  <Modal
    :visible="props.show"
    @update:visible="(v) => emits('update:show', v)"
    centered
    title="Tambahkan Akun Big Seller"
    style="width: 380px"
    ok-text="Simpan"
    cancel-text="Batal"
  >
    <Space style="width: 100%" direction="vertical">
      <Input addon-before="Email" placeholder="son_goku@yahoo.com" />
      <InputPassword placeholder="••••••••" addon-before="Password" />
      <Divider style="margin-block: 4px">
        <TypographyText style="font-size: 13px; font-weight: 400">
          Masukan Captcha
        </TypographyText>
      </Divider>
      <div v-if="pending" class="spinner">
        <Spin size="small" />
      </div>
      <Image
        v-else
        :src="image"
        width="100%"
        height="auto"
        style="border-radius: 2px"
        :preview="false"
      />
      <Input addon-before="Captcha"> </Input>
      <Button block type="primary" ghost>Coba Masuk</Button>
    </Space>
  </Modal>
</template>

<style lang="scss">
.spinner {
  display: flex;
  width: 100%;
  min-height: 100px;
  justify-content: center;
  align-items: center;
}
</style>
