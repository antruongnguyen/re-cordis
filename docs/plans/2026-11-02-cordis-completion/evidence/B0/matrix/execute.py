#!/usr/bin/env python3
"""Accepted B0 matrix with evidence-local logging; no test changes."""
import csv, datetime, hashlib, json, os, platform, shlex, shutil, subprocess, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[6]
HERE = Path(__file__).resolve().parent
UPSTREAM = ROOT / 'local/cordis'
PIN = '35fdc969e4b6c48c48ed44717c7a018324e4dd88'
LOGS = HERE / 'logs'
LOGS.mkdir(exist_ok=True)
RESULTS = []
BASE = os.environ.copy()
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
def output(argv, cwd=ROOT):
    return subprocess.check_output(argv, cwd=cwd, text=True).strip()
def run(label, argv, cwd=ROOT, env=None):
    env = env or BASE
    start = now()
    log = LOGS / (label + '.log')
    if log.exists():
        raise RuntimeError('Refusing to overwrite attempt log: ' + str(log))
    print('START ' + label + ': ' + shlex.join(argv), flush=True)
    with log.open('w') as h:
        h.write('COMMAND: ' + shlex.join(argv) + '\nCWD: ' + str(cwd) + '\nSTART_UTC: ' + start + '\n\n')
        h.flush()
        result = subprocess.run(argv, cwd=cwd, env=env, stdout=h, stderr=subprocess.STDOUT)
        end = now()
        h.write('\nEND_UTC: ' + end + '\nEXIT_CODE: ' + str(result.returncode) + '\n')
    RESULTS.append(dict(id=label, argv=argv, cwd=str(cwd), environment={k:env[k] for k in
        ('PATH','RUSTC','RUSTDOC','CARGO_TARGET_DIR','RUSTDOCFLAGS') if k in env},
        start_utc=start, end_utc=end, exit_code=result.returncode, log=str(log.relative_to(HERE))))
    (HERE/'commands.json').write_text(json.dumps(RESULTS, indent=2)+'\n')
    with (HERE/'commands.tsv').open('w') as h:
        w=csv.writer(h, delimiter='\t')
        w.writerow(['id','command','cwd','environment','start_utc','end_utc','exit_code','log'])
        for r in RESULTS:
            w.writerow([r['id'],shlex.join(r['argv']),r['cwd'],json.dumps(r['environment'],sort_keys=True),r['start_utc'],r['end_utc'],r['exit_code'],r['log']])
    print('END ' + label + ': exit ' + str(result.returncode), flush=True)
    if result.returncode:
        print(log.read_text()[-6000:],flush=True)
        raise RuntimeError('Failed command: ' + label)
def snapshot(phase):
    files=output(['git','ls-files','crates','spikes','tests','Cargo.toml','Cargo.lock','.github/workflows/ci.yml']).splitlines()
    hashes={n:hashlib.sha256((ROOT/n).read_bytes()).hexdigest() for n in files if (ROOT/n).is_file()}
    lock=UPSTREAM/'yarn.lock'
    configs=[p/'.cargo'/n for p in [ROOT,*ROOT.parents] for n in ('config','config.toml')]
    cargo_home=Path(BASE.get('CARGO_HOME',str(Path.home()/'.cargo')))
    configs += [cargo_home/'config',cargo_home/'config.toml']
    data=dict(utc=now(), branch=output(['git','branch','--show-current']),head=output(['git','rev-parse','HEAD']),
        status=output(['git','status','--porcelain=v1']),delta_since_808c9f5=output(['git','diff','--name-only','808c9f5','HEAD']),
        executable_delta_since_808c9f5=output(['git','diff','--name-only','808c9f5','HEAD','--','crates','spikes','tests','Cargo.toml','Cargo.lock','.github/workflows/ci.yml']),
        ts_head=output(['git','rev-parse','HEAD'],UPSTREAM),ts_tracked_status=output(['git','status','--porcelain=v1','--untracked-files=no'],UPSTREAM),
        ts_full_status=output(['git','status','--porcelain=v1'],UPSTREAM), fixture_exists=(UPSTREAM/'packages/core/tests/re-cordis-lifecycle-parity.spec.ts').exists(),
        os=platform.platform(),architecture=platform.machine(),python=sys.version,python_executable=sys.executable,
        tool_locations={n:shutil.which(n) for n in ('rustup','python3','node','corepack','mkfifo','head')},
        unix_prerequisites={n:Path(n).is_file() for n in ('/bin/sh','/bin/kill')},
        inherited_rust_cargo_environment={k:v for k,v in BASE.items() if k in ('CARGO_HOME','RUSTUP_HOME','RUSTC','RUSTDOC','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','RUSTFLAGS','RUSTDOCFLAGS','CARGO_ENCODED_RUSTFLAGS','CARGO_BUILD_RUSTFLAGS','CARGO_TARGET_DIR','CARGO_BUILD_TARGET','CARGO_BUILD_RUSTC','CARGO_BUILD_RUSTC_WRAPPER','CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER')},
        cargo_config_sha256={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in dict.fromkeys(configs) if p.is_file()},
        executable_input_sha256=hashes,untracked_upstream_yarn_lock=dict(exists=lock.exists(),tracked=bool(output(['git','ls-files','--','yarn.lock'],UPSTREAM)),sha256=hashlib.sha256(lock.read_bytes()).hexdigest() if lock.exists() else None))
    (HERE/(phase+'.json')).write_text(json.dumps(data,indent=2,sort_keys=True)+'\n')
    if data['ts_head']!=PIN or data['ts_tracked_status'] or data['fixture_exists']:
        raise RuntimeError('TS pin/cleanliness/exclusive fixture precondition failed')
    return data
def main():
    before=snapshot('before')
    for label,argv,cwd in [('environment-rustup',['rustup','show'],ROOT),('environment-python',['python3','--version'],ROOT),
        ('environment-node',['node','--version'],ROOT),('environment-corepack',['corepack','--version'],ROOT),
        ('environment-yarn',['corepack','yarn','--version'],UPSTREAM)]:
        run(label,argv,cwd)
    versions="const fs=require('fs'),path=require('path');for(const p of ['typescript','eslint','vitest','cosmokit','tsx','yakumo']){try{let d=path.dirname(require.resolve(p));while(d!==path.dirname(d)){const f=path.join(d,'package.json');if(fs.existsSync(f)){const j=JSON.parse(fs.readFileSync(f,'utf8'));if(j.name===p){console.log(p+' '+j.version+' '+f);break}}d=path.dirname(d)}}catch(e){console.log(p+' MISSING '+e.message);process.exitCode=1}}"
    run('environment-js-packages',['corepack','yarn','node','-e',versions],UPSTREAM)
    for tc in ('stable','1.85.0'):
        tools={}
        for n in ('rustc','cargo','rustdoc','cargo-clippy','clippy-driver'):
            run(tc+'-locate-'+n,['rustup','which','--toolchain',tc,n])
            tools[n]=output(['rustup','which','--toolchain',tc,n])
        bindir=str(Path(tools['rustc']).parent)
        if any(str(Path(p).parent)!=bindir for p in tools.values()):
            raise RuntimeError('Binary parent mismatch')
        env=BASE|dict(PATH=bindir+os.pathsep+BASE['PATH'],RUSTC=tools['rustc'],RUSTDOC=tools['rustdoc'],CARGO_TARGET_DIR=str(ROOT/('target/acceptance-'+tc)),RUSTDOCFLAGS='-D warnings')
        (HERE/('environment-'+tc+'.json')).write_text(json.dumps(dict(tools=tools,environment={k:env[k] for k in ('PATH','RUSTC','RUSTDOC','CARGO_TARGET_DIR','RUSTDOCFLAGS')}),indent=2)+'\n')
        for n,p in tools.items():
            run(tc+'-version-'+n,[p,'--version'],env=env)
        cargo=tools['cargo']
        for label,args in [('workspace-check',['check','--workspace','--all-targets','--locked']),('workspace-test',['test','--workspace','--locked']),
            ('workspace-clippy',['clippy','--workspace','--all-targets','--locked','--','-D','warnings']),('workspace-doc',['doc','--workspace','--no-deps','--locked']),
            ('m0-test',['test','--manifest-path','spikes/m0/Cargo.toml','--locked']),('m0-clippy',['clippy','--manifest-path','spikes/m0/Cargo.toml','--all-targets','--locked','--','-D','warnings'])]:
            run(tc+'-'+label,[cargo,*args],env=env)
        if tc=='stable':
            for n in ('cargo-fmt','rustfmt'):
                run(tc+'-locate-'+n,['rustup','which','--toolchain',tc,n])
                p=output(['rustup','which','--toolchain',tc,n])
                if str(Path(p).parent)!=bindir:
                    raise RuntimeError('Formatting binary parent mismatch')
                if n=='rustfmt':
                    run(tc+'-version-'+n,[p,'--version'],env=env)
            run(tc+'-workspace-fmt',[cargo,'fmt','--all','--check'],env=env)
            run(tc+'-m0-fmt',[cargo,'fmt','--manifest-path','spikes/m0/Cargo.toml','--check'],env=env)
        for crate,example in [('re-cordis-core','lifecycle'),('re-cordis-timer','timer_lifecycle')]:
            run(tc+'-example-'+example,[cargo,'run','-p',crate,'--example',example,'--locked'],env=env)
    for label,args in [('docs-regressions',['-B','tests/docs/test_check.py']),('docs-stable',['-B','tests/docs/check.py','--toolchain','stable']),
        ('docs-1.85.0',['-B','tests/docs/check.py','--toolchain','1.85.0']),('parity',['tests/parity/run.py'])]:
        run(label,['python3',*args])
    after=snapshot('after')
    if before['executable_input_sha256']!=after['executable_input_sha256']:
        raise RuntimeError('Executable input hashes changed')
    print('MATRIX COMPLETE: all exits zero; source pin clean; input hashes unchanged.',flush=True)
if __name__=='__main__':
    try:
        main()
    finally:
        snapshot('after')
