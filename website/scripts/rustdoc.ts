// Rustdoc 1.97.1 references these scripts but emits no out-of-crate records.
// This finite set was checked against a complete workspace + dependency build.
export const emptyImplementorScripts = [
  { file: 'trait.impl/futures_task/arc_wake/trait.ArcWake.js', owner: 'futures_task', sources: ['futures_task/trait.ArcWake.html', 'futures_util/task/trait.ArcWake.html'] },
  { file: 'trait.impl/yaml_rust2/parser/trait.EventReceiver.js', owner: 'yaml_rust2', sources: ['yaml_rust2/parser/trait.EventReceiver.html'] },
  { file: 'trait.impl/re_cordis_core/service/trait.ServiceInvoke.js', owner: 're_cordis_core', sources: ['re_cordis_core/trait.ServiceInvoke.html'] },
  { file: 'trait.impl/serde_core/de/trait.VariantAccess.js', owner: 'serde_core', sources: ['serde_core/de/trait.VariantAccess.html'] },
];

export function emptyRegistration(owner: string): string {
  return `(function() {
    const implementors = Object.fromEntries(${JSON.stringify([[owner, []]])});
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()\n`;
}
