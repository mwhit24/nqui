<template>
	<div class="h-full">
		<!-- Static sidebar for desktop -->
		<div class="fixed inset-y-0 flex w-64 flex-col border-r border-gray-200 bg-gray-100 pt-2 pb-4">
			<!-- Sidebar component, swap this element with another sidebar if you like -->
			<div class="flex h-0 flex-1 flex-col overflow-y-auto pt-0">
				<!-- Sidebar Search -->
				<div class="px-3 pb-5 border-b">
					<label class="sr-only" for="search">Search</label>
					<div class="relative mt-1 rounded-sm shadow-sm">
						<div aria-hidden="true" class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
							<MagnifyingGlassIcon aria-hidden="true" class="mr-3 h-4 w-4 text-gray-400" />
						</div>
						<input id="search" class="block w-full rounded-md border-gray-300 pl-9 focus:border-red-500 focus:ring-red-500 sm:text-sm duration-200" name="search" placeholder="Search" type="text" />
					</div>
				</div>
				<!-- Navigation -->
				<nav class="mt-2 px-3">
					<div class="mt-2">
						<!-- Secondary navigation -->
						<h3 id="desktop-teams-headline" class="px-3 text-base font-normal text-black border-b w-full">Tables</h3>
						<div aria-labelledby="desktop-teams-headline" class="mt-1 space-y-1" role="group">
							<div v-if="tables.length > 0">
								<a v-for="(table, tableIndex) in tables" :key="tableIndex"  class="cursor-pointer group flex items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 hover:text-gray-900">
									<span :class="[table.bgColorClass, 'w-2.5 h-2.5 mr-4 rounded-full']" aria-hidden="true" />
									<span class="truncate">{{ table.name }}</span>
								</a>
							</div>
							<div v-else>
								<h3 id="desktop-teams-headline" class="px-3 text-sm font-medium text-gray-500 text-center mt-10">No tables found!</h3>
							</div>

						</div>
					</div>
				</nav>
			</div>
		</div>
		<!-- Main column -->
		<div class="flex flex-col lg:pl-64">
			<!-- Search header -->
<!--			<div class="sticky top-0 z-10 flex h-16 flex-shrink-0 border-b border-gray-200 bg-white lg:hidden">-->
<!--				<button class="border-r border-gray-200 px-4 text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-purple-500 lg:hidden" type="button" @click="sidebarOpen = true">-->
<!--					<span class="sr-only">Open sidebar</span>-->
<!--					<Bars3CenterLeftIcon aria-hidden="true" class="h-6 w-6" />-->
<!--				</button>-->
<!--				<div class="flex flex-1 justify-between px-4 sm:px-6 lg:px-8">-->
<!--					<div class="flex flex-1">-->
<!--						<form action="#" class="flex w-full md:ml-0" method="GET">-->
<!--							<label class="sr-only" for="search-field">Search</label>-->
<!--							<div class="relative w-full text-gray-400 focus-within:text-gray-600">-->
<!--								<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center">-->
<!--									<MagnifyingGlassIcon aria-hidden="true" class="h-5 w-5" />-->
<!--								</div>-->
<!--								<input id="search-field" class="block h-full w-full border-transparent py-2 pl-8 pr-3 text-gray-900 placeholder-gray-500 focus:border-transparent focus:placeholder-gray-400 focus:outline-none focus:ring-0 sm:text-sm" name="search-field" placeholder="Search" type="search" />-->
<!--							</div>-->
<!--						</form>-->
<!--					</div>-->
<!--					<div class="flex items-center">-->
<!--						&lt;!&ndash; Profile dropdown &ndash;&gt;-->
<!--						<Menu as="div" class="relative ml-3">-->
<!--							<div>-->
<!--								<MenuButton class="flex max-w-xs items-center rounded-full bg-white text-sm focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2">-->
<!--									<span class="sr-only">Open user menu</span>-->
<!--									<img alt="" class="h-8 w-8 rounded-full" src="https://images.unsplash.com/photo-1502685104226-ee32379fefbe?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" />-->
<!--								</MenuButton>-->
<!--							</div>-->
<!--							<transition enter-active-class="transition ease-out duration-100" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">-->
<!--								<MenuItems class="absolute right-0 z-10 mt-2 w-48 origin-top-right divide-y divide-gray-200 rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">-->
<!--									<div class="py-1">-->
<!--										<MenuItem v-slot="{ active }">-->
<!--											<a :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-sm']" href="#">View profile</a>-->
<!--										</MenuItem>-->
<!--										<MenuItem v-slot="{ active }">-->
<!--											<a :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-sm']" href="#">Settings</a>-->
<!--										</MenuItem>-->
<!--										<MenuItem v-slot="{ active }">-->
<!--											<a :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-sm']" href="#">Notifications</a>-->
<!--										</MenuItem>-->
<!--									</div>-->
<!--									<div class="py-1">-->
<!--										<MenuItem v-slot="{ active }">-->
<!--											<a :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-sm']" href="#">Get desktop app</a>-->
<!--										</MenuItem>-->
<!--										<MenuItem v-slot="{ active }">-->
<!--											<a :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-sm']" href="#">Support</a>-->
<!--										</MenuItem>-->
<!--									</div>-->
<!--									<div class="py-1">-->
<!--										<MenuItem v-slot="{ active }">-->
<!--											<a :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-sm']" href="#">Logout</a>-->
<!--										</MenuItem>-->
<!--									</div>-->
<!--								</MenuItems>-->
<!--							</transition>-->
<!--						</Menu>-->
<!--					</div>-->
<!--				</div>-->
<!--			</div>-->
			<main class="flex-1">
				<!-- Page title & actions -->
				<div class="border-b border-gray-200 px-4 py-3 flex items-center justify-between">
					<div class="px-3 pb-2 w-full">
						<label class="sr-only" for="search">Run query</label>
						<div class="relative rounded-sm">
							<div aria-hidden="true" class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
								<MagnifyingGlassIcon aria-hidden="true" class="mr-3 h-4 w-4 text-gray-400" />
							</div>
							<input id="search" class="block w-full rounded-md border-gray-300 pl-9 focus:border-red-500 focus:ring-red-500 sm:text-sm duration-200" name="search" placeholder="Run query" type="text" />
						</div>
					</div>
					<div class="mt-4 flex sm:mt-0 sm:ml-4 shadow-sm">
						<button class="sm:order-0 order-1 ml-3 inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 sm:ml-0" type="button">Run</button>
					</div>
				</div>
				<!-- Pinned projects -->
<!--				<div class="mt-6 px-4 sm:px-6 lg:px-8">-->
<!--					<h2 class="text-sm font-medium text-gray-900">Pinned Projects</h2>-->
<!--					<ul class="mt-3 grid grid-cols-1 gap-4 sm:grid-cols-2 sm:gap-6 xl:grid-cols-4" role="list">-->
<!--						<li v-for="project in pinnedProjects" :key="project.id" class="relative col-span-1 flex rounded-md shadow-sm">-->
<!--							<div :class="[project.bgColorClass, 'flex-shrink-0 flex items-center justify-center w-16 text-white text-sm font-medium rounded-l-md']">{{ project.initials }}</div>-->
<!--							<div class="flex flex-1 items-center justify-between truncate rounded-r-md border-t border-r border-b border-gray-200 bg-white">-->
<!--								<div class="flex-1 truncate px-4 py-2 text-sm">-->
<!--									<a class="font-medium text-gray-900 hover:text-gray-600" href="#">{{ project.title }}</a>-->
<!--									<p class="text-gray-500">{{ project.totalMembers }} Members</p>-->
<!--								</div>-->
<!--								<Menu as="div" class="flex-shrink-0 pr-2">-->
<!--									<MenuButton class="inline-flex h-8 w-8 items-center justify-center rounded-full bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2">-->
<!--										<span class="sr-only">Open options</span>-->
<!--										<EllipsisVerticalIcon aria-hidden="true" class="h-5 w-5" />-->
<!--									</MenuButton>-->
<!--									<transition enter-active-class="transition ease-out duration-100" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">-->
<!--										<MenuItems class="absolute right-10 top-3 z-10 mx-3 mt-1 w-48 origin-top-right divide-y divide-gray-200 rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">-->
<!--											<div class="py-1">-->
<!--												<MenuItem v-slot="{ active }">-->
<!--													<a :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-sm']" href="#">View</a>-->
<!--												</MenuItem>-->
<!--											</div>-->
<!--											<div class="py-1">-->
<!--												<MenuItem v-slot="{ active }">-->
<!--													<a :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-sm']" href="#">Removed from pinned</a>-->
<!--												</MenuItem>-->
<!--												<MenuItem v-slot="{ active }">-->
<!--													<a :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-sm']" href="#">Share</a>-->
<!--												</MenuItem>-->
<!--											</div>-->
<!--										</MenuItems>-->
<!--									</transition>-->
<!--								</Menu>-->
<!--							</div>-->
<!--						</li>-->
<!--					</ul>-->
<!--				</div>-->

				<!-- Projects list (only on smallest breakpoint) -->
<!--				<div class="mt-10 sm:hidden">-->
<!--					<div class="px-4 sm:px-6">-->
<!--						<h2 class="text-sm font-medium text-gray-900">Projects</h2>-->
<!--					</div>-->
<!--					<ul class="mt-3 divide-y divide-gray-100 border-t border-gray-200" role="list">-->
<!--						<li v-for="project in projects" :key="project.id">-->
<!--							<a class="group flex items-center justify-between px-4 py-4 hover:bg-gray-50 sm:px-6" href="#">-->
<!--                <span class="flex items-center space-x-3 truncate">-->
<!--                  <span :class="[project.bgColorClass, 'w-2.5 h-2.5 flex-shrink-0 rounded-full']" aria-hidden="true" />-->
<!--                  <span class="truncate text-sm font-medium leading-6">-->
<!--                    {{ project.title }}-->
<!--                    {{ ' ' }}-->
<!--                    <span class="truncate font-normal text-gray-500">in {{ project.team }}</span>-->
<!--                  </span>-->
<!--                </span>-->
<!--								<ChevronRightIcon aria-hidden="true" class="ml-4 h-5 w-5 text-gray-400 group-hover:text-gray-500" />-->
<!--							</a>-->
<!--						</li>-->
<!--					</ul>-->
<!--				</div>-->

				<!-- Projects table (small breakpoint and up) -->
<!--				<div class="mt-8 hidden sm:block">-->
<!--					<div class="inline-block min-w-full border-b border-gray-200 align-middle">-->
<!--						<table class="min-w-full">-->
<!--							<thead>-->
<!--							<tr class="border-t border-gray-200">-->
<!--								<th class="border-b border-gray-200 bg-gray-50 px-6 py-3 text-left text-sm font-semibold text-gray-900" scope="col">-->
<!--									<span class="lg:pl-2">Project</span>-->
<!--								</th>-->
<!--								<th class="border-b border-gray-200 bg-gray-50 px-6 py-3 text-left text-sm font-semibold text-gray-900" scope="col">Members</th>-->
<!--								<th class="hidden border-b border-gray-200 bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900 md:table-cell" scope="col">Last updated</th>-->
<!--								<th class="border-b border-gray-200 bg-gray-50 py-3 pr-6 text-right text-sm font-semibold text-gray-900" scope="col" />-->
<!--							</tr>-->
<!--							</thead>-->
<!--							<tbody class="divide-y divide-gray-100 bg-white">-->
<!--							<tr v-for="project in projects" :key="project.id">-->
<!--								<td class="w-full max-w-0 whitespace-nowrap px-6 py-3 text-sm font-medium text-gray-900">-->
<!--									<div class="flex items-center space-x-3 lg:pl-2">-->
<!--										<div :class="[project.bgColorClass, 'flex-shrink-0 w-2.5 h-2.5 rounded-full']" aria-hidden="true" />-->
<!--										<a class="truncate hover:text-gray-600" href="#">-->
<!--                        <span>-->
<!--                          {{ project.title }}-->
<!--                          {{ ' ' }}-->
<!--                          <span class="font-normal text-gray-500">in {{ project.team }}</span>-->
<!--                        </span>-->
<!--										</a>-->
<!--									</div>-->
<!--								</td>-->
<!--								<td class="px-6 py-3 text-sm font-medium text-gray-500">-->
<!--									<div class="flex items-center space-x-2">-->
<!--										<div class="flex flex-shrink-0 -space-x-1">-->
<!--											<img v-for="member in project.members" :key="member.handle" :alt="member.name" :src="member.imageUrl" class="h-6 w-6 max-w-none rounded-full ring-2 ring-white" />-->
<!--										</div>-->
<!--										<span v-if="project.totalMembers > project.members.length" class="flex-shrink-0 text-xs font-medium leading-5">+{{ project.totalMembers - project.members.length }}</span>-->
<!--									</div>-->
<!--								</td>-->
<!--								<td class="hidden whitespace-nowrap px-6 py-3 text-right text-sm text-gray-500 md:table-cell">{{ project.lastUpdated }}</td>-->
<!--								<td class="whitespace-nowrap px-6 py-3 text-right text-sm font-medium">-->
<!--									<a class="text-indigo-600 hover:text-indigo-900" href="#">Edit</a>-->
<!--								</td>-->
<!--							</tr>-->
<!--							</tbody>-->
<!--						</table>-->
<!--					</div>-->
<!--				</div>-->
			</main>
		</div>
	</div>
</template>

<script setup>
import {onMounted, ref} from 'vue'
import {
	Dialog,
	DialogPanel,
	Menu,
	MenuButton,
	MenuItem,
	MenuItems,
	TransitionChild,
	TransitionRoot,
} from '@headlessui/vue'
import { Bars3CenterLeftIcon, Bars4Icon, ClockIcon, HomeIcon, XMarkIcon } from '@heroicons/vue/24/outline'
import { ChevronRightIcon, ChevronUpDownIcon, EllipsisVerticalIcon, MagnifyingGlassIcon } from '@heroicons/vue/20/solid'
import {invoke} from "@tauri-apps/api/tauri";


const teams = [
	{ name: 'Engineering', href: '#', bgColorClass: 'bg-indigo-500' },
	{ name: 'Human Resources', href: '#', bgColorClass: 'bg-green-500' },
	{ name: 'Customer Success', href: '#', bgColorClass: 'bg-yellow-500' },
]
const projects = [
	{
		id: 1,
		title: 'GraphQL API',
		initials: 'GA',
		team: 'Engineering',
		members: [
			{
				name: 'Dries Vincent',
				handle: 'driesvincent',
				imageUrl:
					'https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
			},
			{
				name: 'Lindsay Walton',
				handle: 'lindsaywalton',
				imageUrl:
					'https://images.unsplash.com/photo-1517841905240-472988babdf9?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
			},
			{
				name: 'Courtney Henry',
				handle: 'courtneyhenry',
				imageUrl:
					'https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
			},
			{
				name: 'Tom Cook',
				handle: 'tomcook',
				imageUrl:
					'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
			},
		],
		totalMembers: 12,
		lastUpdated: 'March 17, 2020',
		pinned: true,
		bgColorClass: 'bg-pink-600',
	},
	// More projects...
]
const pinnedProjects = projects.filter((project) => project.pinned)

const sidebarOpen = ref(false)

const tables = ref([]);
onMounted(async () => {
	// Grab list of tables
await getTables();
})

async function getTables() {
	const dbTables = JSON.parse(await invoke("get_tables"));
	console.log(dbTables.results)
	tables.value = dbTables.results[0].rows;
	tables.value.forEach((table) => {
		table.bgColorClass = getRowColor();
	})
	console.log(tables.value);
}

function getRowColor() {
	const colors = ["bg-indigo-500", "bg-red-500", "bg-yellow-500"];
	return colors[Math.floor(Math.random()*2) + 1]
}
</script>
