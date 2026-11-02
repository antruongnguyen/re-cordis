#!/usr/bin/env python3
"""Derive B0 suite and individual-test inventory from preserved command logs."""
import json,re
from pathlib import Path
HERE=Path(__file__).resolve().parent
results=json.loads((HERE/'commands.json').read_text())
suites=[]
tests=[]
for command in results:
    if not (command['id'].endswith('workspace-test') or command['id'].endswith('m0-test') or command['id']=='parity'):
        continue
    crate='re-cordis-core'
    suite=None
    for line in (HERE/command['log']).read_text().splitlines():
        if 'Running unittests src/lib.rs' in line:
            crate=('re-cordis-timer' if 're_cordis_timer-' in line else
                   're-cordis-m0-spike' if 're_cordis_m0_spike-' in line else 're-cordis-core')
            suite='unit'
        elif (match:=re.search(r'Running tests/(\w+)\.rs',line)):
            suite='tests/'+match[1]+'.rs'
        elif (match:=re.match(r'\s*Doc-tests (\w+)',line)):
            crate=match[1].replace('_','-')
            suite='rustdoc'
        elif (match:=re.match(r'test (.+) \.\.\. (ok|FAILED|ignored)$',line)):
            tests.append(dict(command_id=command['id'],crate=crate,suite=suite,
                              test=match[1],outcome=match[2],compile_fail='(line ' in match[1] and '- compile fail' in match[1]))
        elif (match:=re.match(r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;',line)):
            suites.append(dict(command_id=command['id'],crate=crate,suite=suite,result=match[1],
                              **dict(zip(['passed','failed','ignored','measured','filtered'],map(int,match.groups()[1:])))))
comparisons={}
for name,count in re.findall(r'^([\w-]+): Rust == TypeScript \((\d+) observations\)$',(HERE/'logs/parity.log').read_text(),re.M):
    comparisons[name]=int(count)
assert comparisons==dict(combined=11,coalesced=14,removed=12,**{'superseded-failure':10,'latest-failure':13,'explicit-recovery':13})
assert sum(comparisons.values())==73
assert all(r['exit_code']==0 for r in results)
assert all(s['failed']==s['ignored']==s['filtered']==0 for s in suites)
for tc in ('stable','1.85.0'):
    subset=[s for s in suites if s['command_id']==tc+'-workspace-test']
    assert sum(s['passed'] for s in subset if s['crate']=='re-cordis-core' and s['suite'].startswith('tests/'))==164
    assert sum(s['passed'] for s in subset if s['crate']=='re-cordis-timer' and s['suite'].startswith('tests/'))==15
    assert sum(s['passed'] for s in subset if s['crate']=='re-cordis-core' and s['suite']=='rustdoc')==17
    assert sum(s['passed'] for s in suites if s['command_id']==tc+'-m0-test' and s['suite'].startswith('tests/'))==20
before=json.loads((HERE/'before.json').read_text())
after=json.loads((HERE/'after-supplement.json').read_text())
assert before['executable_input_sha256']==after['executable_input_sha256']
assert before['untracked_upstream_yarn_lock']==after['untracked_upstream_yarn_lock']
assert not before['executable_delta_since_808c9f5'] and not after['executable_delta_since_808c9f5']
(HERE/'results.json').write_text(json.dumps(dict(command_count=len(results),commands_all_exit_zero=True,suites=suites,tests=tests,paired_comparisons=comparisons,input_hash_count=len(before['executable_input_sha256'])),indent=2)+'\n')
print('Validated '+str(len(results))+' zero-exit command logs; '+str(len(suites))+' suite summaries; '+str(len(tests))+' individual test outcomes; six comparisons / 73 observations; '+str(len(before['executable_input_sha256']))+' unchanged executable-input hashes.')
for tc in ('stable','1.85.0'):
    print(tc+' Rustdoc outcomes: '+str([t for t in tests if t['command_id']==tc+'-workspace-test' and t['suite']=='rustdoc']))
