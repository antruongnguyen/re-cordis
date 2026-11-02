import { readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { Context, FiberState } from 'cordis'
import MockLoader from './utils.ts'

function assertTrace(name: string, trace: string[]) {
  const fixtures = process.env.CORDIS_LOADER_PARITY_FIXTURES
  const output = process.env.CORDIS_LOADER_PARITY_OUTPUT
  if (!fixtures || !output) throw new Error('run through tests/loader_parity/run.py')
  const actual = trace.join('\n') + '\n'
  expect(actual).toBe(readFileSync(join(fixtures, `${name}.txt`), 'utf8'))
  writeFileSync(join(output, `${name}.txt`), actual)
}

const state = (value: FiberState) => {
  if (value === FiberState.PENDING) return 'Pending'
  if (value === FiberState.FAILED) return 'Failed'
  if (value === FiberState.ACTIVE) return 'Active'
  throw new Error(`unexpected fiber state: ${value}`)
}

describe('re-cordis paired loader observations', () => {
  it('dependency arrival exposes failure and explicit update recovers', async () => {
    const root = new Context()
    const values: number[] = []
    let releases = 0
    try {
      await root.plugin(MockLoader)
      const loader = root.loader as MockLoader
      loader.modules.worker = {
        inject: ['settings'],
        apply: (_: Context, value: number) => {
          values.push(value)
          if (value < 0) throw new Error('rejected source')
          return () => { releases++ }
        },
      }
      const id = await loader.create({ name: 'worker', config: -1 })
      await loader.await()
      const trace = [`pending state=${state(loader.expectFiber(id).state)} values=${'[' + values.join(', ') + ']'}`]
      root.provide('settings', {})
      await loader.await()
      trace.push(`arrived state=${state(loader.expectFiber(id).state)} values=[${values.join(', ')}]`)
      await loader.update(id, { config: 2 })
      await loader.await()
      trace.push(`recovered state=${state(loader.expectFiber(id).state)} values=[${values.join(', ')}]`)
      loader.remove(id)
      await loader.await()
      trace.push(`removed entries=${[...loader.entries()].length} releases=${releases}`)
      assertTrace('recovery', trace)
    } finally {
      await root.fiber.dispose()
    }
  }, 10_000)

  it('structural disabled group controls child cleanup', async () => {
    const root = new Context()
    let applies = 0
    let releases = 0
    try {
      await root.plugin(MockLoader)
      const loader = root.loader as MockLoader
      loader.mock('worker', () => {
        applies++
        return () => { releases++ }
      })
      const id = await loader.create({
        name: '@cordisjs/plugin-group',
        group: true,
        disabled: true,
        config: [{ id: 'child', name: 'worker' }],
      })
      await loader.await()
      const child = loader.resolve('child')
      const trace: string[] = []
      for (const [phase, disabled] of [['created', true], ['enabled', false], ['disabled', true], ['reenabled', false]] as const) {
        if (phase !== 'created') {
          await loader.update(id, { disabled })
          await loader.await()
        }
        expect(loader.resolve('child')).toBe(child)
        const childState = child.disabled ? 'Disabled' : state(loader.expectFiber('child').state)
        trace.push(`${phase} group=${state(loader.expectFiber(id).state)} child=${childState}`
          + ` entries=${[...loader.entries()].length} applies=${applies} releases=${releases}`)
      }
      await root.fiber.dispose()
      trace.push(`disposed applies=${applies} releases=${releases}`)
      assertTrace('group', trace)
    } finally {
      await root.fiber.dispose()
    }
  }, 10_000)
})
