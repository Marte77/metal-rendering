//
//  Renderer.swift
//  metal-rendering
//
//  Created by Martinho Tavares Malhão on 21/01/2023.
//

import Metal
import MetalKit


// este objeto vai ser responsavel por renderizer sempre que o mtkview pedir para renderizar. assim ate fica mais portatil (para iOS presumo eu)
class Renderer : NSObject, MTKViewDelegate {
    
    let device: MTLDevice //representa a GPU a ser usada
    let commandQueue: MTLCommandQueue // o metal guarda os comandos que vamos chamar na gpu numa queue, é esta
    let pipelineState: MTLRenderPipelineState //vai guardar o estado da nossa pipeline
    let vertexBuffer: MTLBuffer //pool de memoria acessivel a cpu e gpu
    
    
    //construtor do objeto, passamos a view
    init?(mtkView:MTKView){
        device = mtkView.device!
        commandQueue = device.makeCommandQueue()!
        
        var vertices = [Vertex2D(color: [1, 0, 0, 1], pos: [-1, -1]),
                        Vertex2D(color: [0, 1, 0, 1], pos: [0, 0.5]),
                    Vertex2D(color: [0, 0, 1, 1], pos: [1, -1])]
        let tamanhoRet: Float = 0.5
        vertices.append(contentsOf: [
            Vertex2D(color: [1, 1, 1, 1], pos: [tamanhoRet, tamanhoRet]),
            Vertex2D(color: [1, 1, 1, 1], pos: [-tamanhoRet, -tamanhoRet]),
            Vertex2D(color: [1, 1, 1, 1], pos: [-tamanhoRet, tamanhoRet]),
        ])
        vertices.append(contentsOf: [
            Vertex2D(color: [1, 1, 1, 1], pos: [tamanhoRet, tamanhoRet]),
            Vertex2D(color: [1, 1, 1, 1], pos: [-tamanhoRet, -tamanhoRet]),
            Vertex2D(color: [1, 1, 1, 1], pos: [tamanhoRet, -tamanhoRet]),
        ])
        /*for var v in vertices {
            v.pos = v.pos * SIMD2<Float>(-1,-1)
        }*/
        vertexBuffer = device.makeBuffer(bytes: vertices, length: vertices.count * MemoryLayout<Vertex2D>.stride, options: [])!
        
        
        do {
            pipelineState = try Renderer.buildRenderPipelineWith(device: device, metalKitView: mtkView, vertexFunctionName: "vertexShader2D", fragmentFunctionName: "fragmentShader2D")
        } catch {
            print("Couldnt compile render pipeline state: \(error)")
            return nil
        }
    }
    
    //esta funcao e'chamada sempre que a view muda de tamanho (resize da janela)
    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
        
    }
    
    //sempre que o mtkView quiser renderizar esta funcao vai ser chamada aka desenhar uma frame
    func draw(in view: MTKView) {
        guard let commandBuffer = commandQueue.makeCommandBuffer() else { return } //vamos configurar este buffer e mete-lo na queue de commands
        
        // este descriptor vai configurar certas coisas sobre input e output na gpu e depois com um commandenconder dizemos quais operacoes para desenhar fazemos
        guard let renderPassDescriptor = view.currentRenderPassDescriptor else { return } //em vez de configurarmos nós, usamos as configs defaults que vem da view
        
        renderPassDescriptor.colorAttachments[0].clearColor = MTLClearColorMake(255/255, 16/255, 240/255, 1)
        
        //aqui estamos a compilar/converter o descriptor para o commandencoder, este encoder vai preparar os dados dos vertices assim como os comandos que vao desenhar no ecra, tudo isto antes de meter na pipeline que o descriptor configura
        guard let renderEncoder = commandBuffer.makeRenderCommandEncoder(descriptor: renderPassDescriptor) else { return }
        
        // com isto dizemos que pipeline e que a gpu deve usar
        renderEncoder.setRenderPipelineState(pipelineState)
        // indicamos qual e o buffer de vertices a usar, index:0 pois o metal suporta varios buffers duma vez, so temos um, passamos com 0
        renderEncoder.setVertexBuffer(vertexBuffer, offset: 0, index: 0)
        
        //dizemos o que e que a gpu deve desenhar (existem .line, .point, .lineStrip, .triangleStrip), comecamos na posicao 0 do vertexBuffer
        renderEncoder.drawPrimitives(type: .triangle, vertexStart: 0, vertexCount: vertexBuffer.length)
        
        //termina o encoding dos comandos que desenham, mas NAO envia as informacoes para a gpu, temos que ser nos a dizer quando queremos mandar
        renderEncoder.endEncoding()
        
        //quando a renderizacao terminar, isto vai dizer para a gpu enviar o resultado para a nossa view
        commandBuffer.present(view.currentDrawable!) //o drawable e um recurso do MTKView onde a gpu consegue escrever
        
        //enviamos o commandBuffer para a gpu (se houver outros comandos na commandQueue, este commit vai meter os comandos acabados de criar a serem executados depois dos que já existem na Queue
        commandBuffer.commit()
        
        
    }
    
    class func buildRenderPipelineWith(device: MTLDevice, metalKitView: MTKView, vertexFunctionName: String, fragmentFunctionName: String) throws -> MTLRenderPipelineState {
        let pipelineDescriptor = MTLRenderPipelineDescriptor()
        
        let library = device.makeDefaultLibrary()
        pipelineDescriptor.vertexFunction = library?.makeFunction(name: vertexFunctionName)
        pipelineDescriptor.fragmentFunction = library?.makeFunction(name: fragmentFunctionName)
        
        pipelineDescriptor.colorAttachments[0].pixelFormat = metalKitView.colorPixelFormat
        return try device.makeRenderPipelineState(descriptor: pipelineDescriptor)
    }
}
