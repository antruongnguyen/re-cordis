import { readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it, vi } from 'vitest'
import { Context } from 'cordis'
import Timer from '../src/index.js'

function assertTrace(name: string, trace: string[]) {
  const fixtures = process.env.CORDIS_TIMER_PARITY_FIXTURES
  const output = process.env.CORDIS_TIMER_PARITY_OUTPUT
  if (!fixtures || !output) throw new Error('run through tests/timer_parity/run.py')
  const actual = trace.join('\n') + '\n'
  expect(actual).toBe(readFileSync(join(fixtures, `${name}.txt`), 'utf8'))
  writeFileSync(join(output, `${name}.txt`), actual)
}

function withContext(callback: (ctx: Context) => Promise<void>) {
  return async () => {
    vi.useFakeTimers({ now: 0 })
    const root = new Context()
    try {
      await root.plugin(Timer)
      await root.plugin({ inject: ['timer'], apply: callback })
    } finally {
      await root.fiber.dispose()
      vi.useRealTimers()
    }
  }
}

describe('re-cordis paired timer observations', () => {
  it('timeout fires once', withContext(async (ctx) => {
    let count = 0
    ctx.timeout(() => count++, 10)
    const trace = [`scheduled count=${count}`]
    await vi.advanceTimersByTimeAsync(5)
    trace.push(`advance 5 count=${count}`)
    await vi.advanceTimersByTimeAsync(5)
    trace.push(`advance 10 count=${count}`)
    await vi.advanceTimersByTimeAsync(10)
    trace.push(`advance 20 count=${count}`)
    assertTrace('timeout', trace)
  }))

  it('explicit cancellation prevents timeout', withContext(async (ctx) => {
    let count = 0
    const cancel = ctx.timeout(() => count++, 10)
    const trace = [`scheduled count=${count}`]
    cancel()
    trace.push('cancelled')
    await vi.advanceTimersByTimeAsync(20)
    trace.push(`advance 20 count=${count}`)
    assertTrace('cancel', trace)
  }))

  it('sleep is awaited after resolution', withContext(async (ctx) => {
    let resolved = false
    const sleep = ctx.timeout(10).then(() => { resolved = true })
    const trace = [`scheduled ${resolved ? 'resolved' : 'pending'}`]
    await vi.advanceTimersByTimeAsync(5)
    trace.push(`advance 5 ${resolved ? 'resolved' : 'pending'}`)
    await vi.advanceTimersByTimeAsync(5)
    trace.push(`advance 10 ${resolved ? 'resolved' : 'pending'}`)
    await sleep
    trace.push('await resolved')
    assertTrace('sleep', trace)
  }))

  it('interval has a delayed first tick and stops', withContext(async (ctx) => {
    let count = 0
    const stop = ctx.interval(() => count++, 10)
    const trace = [`scheduled count=${count}`]
    await vi.advanceTimersByTimeAsync(10)
    trace.push(`advance 10 count=${count}`)
    await vi.advanceTimersByTimeAsync(10)
    trace.push(`advance 20 count=${count}`)
    stop()
    trace.push('stopped')
    await vi.advanceTimersByTimeAsync(20)
    trace.push(`advance 40 count=${count}`)
    assertTrace('interval', trace)
  }))
})
