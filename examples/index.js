const { getWorkspaces } = require('../');

console.time('getWorkspaces');

for (let workspace of getWorkspaces()) {
  console.log(`Package: ${workspace.packageJson.name} from ${workspace.dir}`);
}

console.timeEnd('getWorkspaces');