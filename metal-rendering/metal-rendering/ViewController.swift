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
    }

    override var representedObject: Any? {
        didSet {
        // Update the view, if already loaded.
        }
    }


}

