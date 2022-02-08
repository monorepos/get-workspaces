const { globRequire } = require('./lib/index.node');
const findUp = require('find-up');
const path = require('path');

/**
 * @typedef {Object} Workspace
 * @property {string} dir
 * @property {Record<string, any>} packageJson
 */

/**
 * Gets workspaces from the cwd
 * @param {string} cwd 
 * @returns {Workspace[]}
 */
module.exports.getWorkspaces = (cwd = process.cwd()) => {
  const rootPkg = require(findUp.sync('package.json', { cwd }));
  let globs = rootPkg.workspaces.packages || rootPkg.workspaces;
  
  let workspaces = [];

  for (const glob of globs) {
    console.log(path.join(cwd, glob))
    const workspaces = globRequire(path.join(cwd, glob, "package.json"));
    const workspacesJSON = JSON.parse(workspaces);
    worspaces = workspaces.concat(workspacesJSON.workspaces);
  }


  return workspaces;
};

