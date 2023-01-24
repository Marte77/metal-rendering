//
//  ViewController.swift
//  metal-rendering
//
//  Created by Martinho Tavares Malhão on 18/01/2023.
//

import Cocoa
import Metal
import MetalKit

class ViewController: NSViewController {

    var mtkView: MTKView!
    var renderer: Renderer!
    
    override func viewDidLoad() {
        super.viewDidLoad()

        guard let mtkViewTemp = self.view as? MTKView else { print("view attached is not mtkview!");return }
        mtkView = mtkViewTemp // inicializar a nossa view
        
        //obter o objeto da GPU
        guard let defaultDevice = MTLCreateSystemDefaultDevice() else { print("metal not supported on this device");return }
        mtkView.device = defaultDevice
        print("my gpu is \(defaultDevice)")
        
        guard let tempRenderer = Renderer(mtkView: mtkView) else { print("renderer failed to init"); return }
        renderer = tempRenderer
        mtkView.delegate = renderer
        
        NSEvent.addLocalMonitorForEvents(matching: .keyDown, handler: onKeyDown)
        NSEvent.addLocalMonitorForEvents(matching: .flagsChanged, handler: flagsChanged)
    }

    override var representedObject: Any? {
        didSet {
        // Update the view, if already loaded.
        }
    }


    func httpGETRequest() {
        let url = URL(string: "https://jsonplaceholder.typicode.com/todos/1")
        let task = URLSession.shared.dataTask(with: url!) { (data, res, err) in
            print(String(data: data!, encoding: .utf8)!)
            if let httpResponse = res as? HTTPURLResponse {
                    print(httpResponse.statusCode)
            }
        }
        task.resume()
    }
    
    func onKeyDown(event: NSEvent) -> NSEvent? {
        
        switch event.keyCode {
        case 0:
            print("a")
            break
        case 1:
            print("s")
            break
        case 2:
            print("d")
            break
        case 12:
            print("q")
            break
        case 13:
            print("w")
            break
        case 14:
            print("e")
            break
        case 36:
            print("enter")
            break
        case 49:
            print("espaco")
            break
        
        default:
            print(event.keyCode)
            return event
        }
        return nil //retornando nil ja nao faz o alert sound
    }
    
    func flagsChanged(with event: NSEvent) -> NSEvent? {
           switch event.modifierFlags.intersection(.deviceIndependentFlagsMask) {
           case [.shift]:
               print("shift key is pressed")
           case [.control]:
               print("control key is pressed")
           case [.option] :
               print("option key is pressed")
           case [.command]:
               print("Command key is pressed")
           case [.control, .shift]:
               print("control-shift keys are pressed")
           case [.option, .shift]:
               print("option-shift keys are pressed")
           case [.command, .shift]:
               print("command-shift keys are pressed")
           case [.control, .option]:
               print("control-option keys are pressed")
           case [.control, .command]:
               print("control-command keys are pressed")
           case [.option, .command]:
               print("option-command keys are pressed")
           case [.shift, .control, .option]:
               print("shift-control-option keys are pressed")
           case [.shift, .control, .command]:
               print("shift-control-command keys are pressed")
           case [.control, .option, .command]:
               print("control-option-command keys are pressed")
           case [.shift, .command, .option]:
               print("shift-command-option keys are pressed")
           case [.shift, .control, .option, .command]:
               print("shift-control-option-command keys are pressed")
           default:
               print("no modifier keys are pressed")
           }
        return nil
       }
}

