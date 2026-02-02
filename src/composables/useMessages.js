import { watch } from 'vue';

export function useMessages(errorRef, successRef) {
  watch(errorRef, (newError) => {
    if (newError !== null) {
      setTimeout(() => {
        errorRef.value = null;
      }, 60000); // 1 minute
    }
  });

  watch(successRef, (newSuccess) => {
    if (newSuccess !== null) {
      setTimeout(() => {
        successRef.value = null;
      }, 60000);
    }
  });
}
