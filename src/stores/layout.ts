import { defineStore } from "pinia";

export const useLayoutStore = defineStore('layout', {
    state: () => ({
        leftDrawerOpen: false,
        rightDrawerOpen: false,
    }),
    getters: {
        getLeftDrawerOpen: (state) => {
            state.leftDrawerOpen
        },
        getRightDrawerOpen: (state) => {
            state.rightDrawerOpen;
        } 
    },
    actions: {
        setLeftDrawerOpen(value: boolean): void {
            this.leftDrawerOpen = value;
        },
        setRightDrawerOpen(value: boolean): void {
            this.rightDrawerOpen = value;
        }
    }
})
