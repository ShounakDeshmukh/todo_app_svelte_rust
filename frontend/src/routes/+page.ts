import type { PageLoad } from "./$types"


export const load:PageLoad =async () => ({
    todos: (await fetch("http://0.0.0.0:3458").then((data) => data.json())) as Todo[]
});