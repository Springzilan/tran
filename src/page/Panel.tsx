import "../css/Panel.css"

import { invoke } from "@tauri-apps/api"
import { getCurrent, PhysicalPosition } from "@tauri-apps/api/window"
import { createSignal } from "solid-js"

import { CopyIcon, PinIcon } from "../icon"

const Panel = () => {
    const panel = getCurrent()

    let pinFlag = false
    let moveFlag = true

    const [pin, Pin] = createSignal(false)
    const [copy, Copy] = createSignal(false)
    const [result, Result] = createSignal("翻译中...")

    // 监听事件， 显示panel
    panel.listen<{ x: number; y: number; context: string }>(
        "show",
        async (pos) => {
            Result("翻译中...")
            if (!pinFlag) {
                await panel.setPosition(
                    new PhysicalPosition(pos.payload.x - 40, pos.payload.y + 20)
                )

                // 刷新 固定图标状态
                moveFlag = true
                pinFlag = false
                Pin(false)
            }

            Copy(false)
            await panel.show()
            Result(
                await invoke<string>("translate", {
                    context: pos.payload.context,
                })
            )
        }
    )

    return (
        <div
            class="panel"
            onMouseLeave={async () => {
                if (moveFlag) {
                    moveFlag = true
                    pinFlag = false
                    Pin(false)
                    await panel.hide()
                    Result("翻译中...")
                }
            }}>
            <div class="result">{result()}</div>
            <div class="panel-control">
                <div
                    data-tauri-drag-region
                    class="panel-control-item panel-control-pin-conainer"
                    onMouseUp={() => {
                        moveFlag = !moveFlag
                        pinFlag = !pinFlag
                        Pin(pinFlag)
                    }}
                    onMouseEnter={() => {
                        moveFlag = false
                        pinFlag = true
                        Pin(true)
                    }}>
                    <div classList={{ "panel-control-pin": pin() }}>
                        <PinIcon size={14} />
                    </div>
                </div>

                <div
                    class="panel-control-item"
                    classList={{ "panel-control-copy": copy() }}
                    onMouseEnter={() => {
                        // TODO 复制到剪贴板
                        Copy(true)
                    }}>
                    <CopyIcon size={12} />
                </div>
            </div>
        </div>
    )
}

export default Panel
