export function groups(group_count: number): [] {
  let now = Math.trunc(new Date().getTime() / 1000);
  let day = 24 * 60 * 60;
  let groups = [];
  for (let i = 0; i < group_count; i++) {
    let ago = day * 7;

    let start = now - ago * (i + 1);
    let stop = now - ago * i;
    groups.push([start, stop]);
  }
  return { start: groups[groups.length - 1][0], stop: now, groups: groups };
}
