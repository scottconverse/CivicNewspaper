import sqlite3, json, sys
path=sys.argv[1]
conn=sqlite3.connect(path)
conn.row_factory=sqlite3.Row
cur=conn.cursor()
def exists(name):
    return cur.execute("select count(*) c from sqlite_master where type='table' and name=?", (name,)).fetchone()['c']>0
def count(name):
    return cur.execute(f"select count(*) c from {name}").fetchone()['c'] if exists(name) else None
def cols(name):
    return [r['name'] for r in cur.execute(f"pragma table_info({name})")] if exists(name) else []
out={'db':path,'tables':[r['name'] for r in cur.execute("select name from sqlite_master where type='table' order by name")], 'counts':{}, 'columns':{}}
for t in out['tables']:
    out['counts'][t]=count(t)
    out['columns'][t]=cols(t)
for t in ['leads','lead_evidence','drafts','published_posts','sources','scan_runs','scan_evidence','stories']:
    if exists(t):
        out[t+'_sample']=[dict(r) for r in cur.execute(f"select * from {t} limit 20")]
if exists('leads'):
    c=cols('leads')
    wanted=[x for x in ['id','title','headline','source_name','story_type','disposition','priority','status','summary','body','rationale','why_now','suggested_next_step'] if x in c]
    out['leads_selected']=[dict(r) for r in cur.execute('select '+','.join(wanted)+' from leads order by id desc limit 30')] if wanted else []
if exists('lead_evidence'):
    c=cols('lead_evidence')
    wanted=[x for x in ['id','lead_id','source_id','source_name','url','title','snippet','summary','evidence_type'] if x in c]
    out['lead_evidence_selected']=[dict(r) for r in cur.execute('select '+','.join(wanted)+' from lead_evidence limit 50')] if wanted else []
print(json.dumps(out, indent=2, default=str))
